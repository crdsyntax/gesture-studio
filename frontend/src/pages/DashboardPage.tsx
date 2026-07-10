import { useEffect, useState } from "react";
import {
  Grid, Card, CardContent, Typography, Button, Box, Alert,
} from "@mui/material";
import { invoke } from "@tauri-apps/api/core";
import VideocamIcon from "@mui/icons-material/Videocam";
import VideocamOffIcon from "@mui/icons-material/VideocamOff";
import PanToolIcon from "@mui/icons-material/PanTool";
import SpeedIcon from "@mui/icons-material/Speed";
import MemoryIcon from "@mui/icons-material/Memory";
import CameraPreview from "../components/CameraPreview";

interface AppStatus {
  version: string;
  camera_active: boolean;
  model_loaded: boolean;
  fps: number;
  latency_ms: number;
}

interface StorageStats {
  gesture_count: number;
  sample_count: number;
  action_count: number;
}

export default function DashboardPage() {
  const [status, setStatus] = useState<AppStatus | null>(null);
  const [stats, setStats] = useState<StorageStats | null>(null);
  const [error, setError] = useState("");

  const refresh = () => {
    invoke<AppStatus>("get_app_status").then(setStatus).catch(console.error);
    invoke<StorageStats>("get_stats").then(setStats).catch(console.error);
  };

  useEffect(refresh, []);

  const toggleCamera = async () => {
    setError("");
    try {
      if (status?.camera_active) {
        await invoke("stop_camera");
      } else {
        const config = await invoke<any>("get_config");
        const deviceId = config.camera.preferred_device || "default";
        await invoke("start_camera", { deviceId });
      }
      setTimeout(refresh, 200);
    } catch (e) {
      setError(String(e));
    }
  };

  return (
    <Grid container spacing={3}>
      <Grid item xs={12}>
        <Box sx={{ display: "flex", justifyContent: "space-between", alignItems: "center" }}>
          <Box>
            <Typography variant="h4">Dashboard</Typography>
            <Typography variant="body2" color="text.secondary">
              v{status?.version ?? "..."}
            </Typography>
          </Box>
          <Button
            variant="contained"
            color={status?.camera_active ? "error" : "primary"}
            startIcon={status?.camera_active ? <VideocamOffIcon /> : <VideocamIcon />}
            onClick={toggleCamera}
          >
            {status?.camera_active ? "Stop Camera" : "Start Camera"}
          </Button>
        </Box>
      </Grid>

      {error && <Grid item xs={12}><Alert severity="error">{error}</Alert></Grid>}

      <Grid item xs={12} md={8}>
        <CameraPreview height={400} />
      </Grid>

      <Grid item xs={12} md={4}>
        <Grid container spacing={2}>
          <Grid item xs={12}>
            <Card>
              <CardContent>
                <VideocamIcon color={status?.camera_active ? "success" : "disabled"} />
                <Typography variant="h6">Camera</Typography>
                <Typography variant="body2" color="text.secondary">
                  {status?.camera_active ? "Active" : "Inactive"}
                </Typography>
              </CardContent>
            </Card>
          </Grid>
          <Grid item xs={12}>
            <Card>
              <CardContent>
                <PanToolIcon color={status?.model_loaded ? "success" : "disabled"} />
                <Typography variant="h6">Model</Typography>
                <Typography variant="body2" color="text.secondary">
                  {status?.model_loaded ? "Loaded" : "Not loaded"}
                </Typography>
              </CardContent>
            </Card>
          </Grid>
          <Grid item xs={12}>
            <Card>
              <CardContent>
                <SpeedIcon />
                <Typography variant="h6">Performance</Typography>
                <Typography variant="body2" color="text.secondary">
                  {status?.fps.toFixed(1) ?? "0"} FPS / {status?.latency_ms.toFixed(1) ?? "0"} ms
                </Typography>
              </CardContent>
            </Card>
          </Grid>
          <Grid item xs={12}>
            <Card>
              <CardContent>
                <MemoryIcon />
                <Typography variant="h6">Storage</Typography>
                <Typography variant="body2" color="text.secondary">
                  {stats?.gesture_count ?? 0} gestures / {stats?.sample_count ?? 0} samples
                </Typography>
              </CardContent>
            </Card>
          </Grid>
        </Grid>
      </Grid>
    </Grid>
  );
}
