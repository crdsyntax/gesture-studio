import { useEffect, useRef, useCallback } from "react";
import { Box, Typography, Paper } from "@mui/material";
import { listen, UnlistenFn } from "@tauri-apps/api/event";

interface LandmarkPoint {
  x: number;
  y: number;
  z: number;
}

interface NormalizedLandmarks {
  hand_id: number;
  landmarks: LandmarkPoint[];
  wrist: LandmarkPoint;
  thumb_tip: LandmarkPoint;
  index_tip: LandmarkPoint;
  middle_tip: LandmarkPoint;
  ring_tip: LandmarkPoint;
  pinky_tip: LandmarkPoint;
}

const CONNECTIONS: [number, number][] = [
  [0, 1], [1, 2], [2, 3], [3, 4],
  [0, 5], [5, 6], [6, 7], [7, 8],
  [0, 9], [9, 10], [10, 11], [11, 12],
  [0, 13], [13, 14], [14, 15], [15, 16],
  [0, 17], [17, 18], [18, 19], [19, 20],
  [5, 9], [9, 13], [13, 17],
];

const FINGER_TIPS = [4, 8, 12, 16, 20];
const FINGER_COLORS = ["#ff6b6b", "#ffd93d", "#6bcb77", "#4d96ff", "#ff6bff"];
const LANDMARK_COLOR = "#00ff88";
const CONNECTION_COLOR = "rgba(0, 255, 136, 0.5)";

interface CameraPreviewProps {
  height?: number;
}

export default function CameraPreview({ height = 360 }: CameraPreviewProps) {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const overlayRef = useRef<HTMLCanvasElement>(null);
  const landmarksRef = useRef<NormalizedLandmarks[]>([]);
  const animFrameRef = useRef<number>(0);

  const drawLandmarks = useCallback(() => {
    const canvas = overlayRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext("2d");
    if (!ctx) return;

    ctx.clearRect(0, 0, canvas.width, canvas.height);

    const w = canvas.width;
    const h = canvas.height;
    const scale = Math.min(w, h) * 0.7;
    const cx = w / 2;
    const cy = h / 2;

    for (const nl of landmarksRef.current) {
      for (const [i, j] of CONNECTIONS) {
        const p1 = nl.landmarks[i];
        const p2 = nl.landmarks[j];
        if (!p1 || !p2) continue;
        ctx.beginPath();
        ctx.moveTo(cx + p1.x * scale, cy + p1.y * scale);
        ctx.lineTo(cx + p2.x * scale, cy + p2.y * scale);
        ctx.strokeStyle = CONNECTION_COLOR;
        ctx.lineWidth = 2;
        ctx.stroke();
      }

      for (let i = 0; i < nl.landmarks.length; i++) {
        const p = nl.landmarks[i];
        const sx = cx + p.x * scale;
        const sy = cy + p.y * scale;
        const isTip = FINGER_TIPS.includes(i);
        const radius = isTip ? 6 : 4;
        const color = isTip
          ? FINGER_COLORS[FINGER_TIPS.indexOf(i) % FINGER_COLORS.length]
          : LANDMARK_COLOR;

        ctx.beginPath();
        ctx.arc(sx, sy, radius, 0, Math.PI * 2);
        ctx.fillStyle = color;
        ctx.fill();
        if (isTip) {
          ctx.strokeStyle = "white";
          ctx.lineWidth = 1.5;
          ctx.stroke();
        }
      }
    }

    animFrameRef.current = requestAnimationFrame(drawLandmarks);
  }, []);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const overlay = overlayRef.current;
    if (!overlay) return;

    canvas.width = canvas.clientWidth * window.devicePixelRatio;
    canvas.height = canvas.clientHeight * window.devicePixelRatio;
    overlay.width = canvas.width;
    overlay.height = canvas.height;

    const unlisteners: UnlistenFn[] = [];

    const setup = async () => {
      const u1 = await listen<{ data: string; width: number; height: number }>(
        "camera-frame",
        (event) => {
          const ctx = canvas.getContext("2d");
          if (!ctx) return;
          const img = new Image();
          img.onload = () => {
            ctx.drawImage(img, 0, 0, canvas.width, canvas.height);
          };
          img.src = `data:image/jpeg;base64,${event.payload.data}`;
        },
      );
      unlisteners.push(u1);

      const u2 = await listen<NormalizedLandmarks>("landmarks-normalized", (event) => {
        landmarksRef.current = [event.payload];
      });
      unlisteners.push(u2);

      const u3 = await listen("hands-lost", () => {
        landmarksRef.current = [];
      });
      unlisteners.push(u3);
    };

    setup();
    animFrameRef.current = requestAnimationFrame(drawLandmarks);

    return () => {
      unlisteners.forEach((u) => u());
      cancelAnimationFrame(animFrameRef.current);
    };
  }, [drawLandmarks]);

  return (
    <Paper sx={{ position: "relative", overflow: "hidden", bgcolor: "#000" }}>
      <Box sx={{ position: "relative", width: "100%", height }}>
        <canvas
          ref={canvasRef}
          style={{ width: "100%", height: "100%", display: "block" }}
        />
        <canvas
          ref={overlayRef}
          style={{
            position: "absolute",
            top: 0,
            left: 0,
            width: "100%",
            height: "100%",
            pointerEvents: "none",
          }}
        />
      </Box>
      <Typography
        variant="caption"
        sx={{
          position: "absolute",
          bottom: 8,
          right: 12,
          color: "rgba(255,255,255,0.6)",
          fontFamily: "monospace",
        }}
      >
        Camera Preview
      </Typography>
    </Paper>
  );
}
