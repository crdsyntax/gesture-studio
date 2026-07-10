import { useEffect, useState } from "react";
import {
  Typography, Box, Paper, Button, TextField, Grid, Slider, Alert, Select, MenuItem, InputLabel, FormControl,
} from "@mui/material";
import SaveIcon from "@mui/icons-material/Save";
import { invoke } from "@tauri-apps/api/core";

interface CameraDevice {
  id: string;
  name: string;
}

interface AppConfig {
  camera: { preferred_device: string | null; resolution: { width: number; height: number }; fps: number };
  vision: { model_path: string; confidence_threshold: number; max_hands: number };
  recognition: { static_threshold: number; dynamic_threshold: number; smoothing_factor: number };
  storage: { db_path: string; max_samples_per_gesture: number };
  performance: { target_fps: number; buffer_size: number; log_level: string };
}

export default function SettingsPage() {
  const [config, setConfig] = useState<AppConfig | null>(null);
  const [cameras, setCameras] = useState<CameraDevice[]>([]);
  const [saved, setSaved] = useState(false);

  useEffect(() => {
    invoke<AppConfig>("get_config").then(setConfig).catch(console.error);
    invoke<CameraDevice[]>("list_cameras").then(setCameras).catch(console.error);
  }, []);

  const handleSave = async () => {
    if (!config) return;
    try {
      await invoke("update_config", { config });
      setSaved(true);
      setTimeout(() => setSaved(false), 2000);
    } catch (e) {
      console.error(e);
    }
  };

  if (!config) return <Typography>Loading...</Typography>;

  return (
    <Box>
      <Box sx={{ display: "flex", justifyContent: "space-between", mb: 2 }}>
        <Typography variant="h4">Settings</Typography>
        <Button startIcon={<SaveIcon />} variant="contained" onClick={handleSave}>
          Save
        </Button>
      </Box>

      {saved && <Alert severity="success" sx={{ mb: 2 }}>Settings saved</Alert>}

      <Grid container spacing={2}>
        <Grid item xs={12} md={6}>
          <Paper sx={{ p: 2 }}>
            <Typography variant="h6" gutterBottom>Camera</Typography>
            <FormControl fullWidth size="small" sx={{ mb: 2 }}>
              <InputLabel>Camera Device</InputLabel>
              <Select
                value={config.camera.preferred_device ?? ""}
                label="Camera Device"
                onChange={(e) => setConfig({
                  ...config,
                  camera: { ...config.camera, preferred_device: e.target.value || null },
                })}
              >
                <MenuItem value="">-- Auto / Default --</MenuItem>
                {cameras.map((cam) => (
                  <MenuItem key={cam.id} value={cam.id}>{cam.name}</MenuItem>
                ))}
              </Select>
            </FormControl>
            <Grid container spacing={1}>
              <Grid item xs={6}>
                <TextField label="Width" type="number" size="small" fullWidth
                  value={config.camera.resolution.width}
                  onChange={(e) => setConfig({
                    ...config,
                    camera: { ...config.camera, resolution: { ...config.camera.resolution, width: Number(e.target.value) } },
                  })} />
              </Grid>
              <Grid item xs={6}>
                <TextField label="Height" type="number" size="small" fullWidth
                  value={config.camera.resolution.height}
                  onChange={(e) => setConfig({
                    ...config,
                    camera: { ...config.camera, resolution: { ...config.camera.resolution, height: Number(e.target.value) } },
                  })} />
              </Grid>
            </Grid>
            <TextField label="FPS" type="number" size="small" fullWidth sx={{ mt: 2 }}
              value={config.camera.fps}
              onChange={(e) => setConfig({ ...config, camera: { ...config.camera, fps: Number(e.target.value) } })} />
          </Paper>
        </Grid>

        <Grid item xs={12} md={6}>
          <Paper sx={{ p: 2 }}>
            <Typography variant="h6" gutterBottom>Vision</Typography>
            <TextField label="Model Path" fullWidth size="small" sx={{ mb: 2 }}
              value={config.vision.model_path}
              onChange={(e) => setConfig({ ...config, vision: { ...config.vision, model_path: e.target.value } })} />
            <Typography gutterBottom>Confidence Threshold: {config.vision.confidence_threshold}</Typography>
            <Slider min={0} max={1} step={0.05} value={config.vision.confidence_threshold}
              onChange={(_, v) => setConfig({ ...config, vision: { ...config.vision, confidence_threshold: v as number } })} />
            <TextField label="Max Hands" type="number" size="small" fullWidth sx={{ mt: 2 }}
              value={config.vision.max_hands}
              onChange={(e) => setConfig({ ...config, vision: { ...config.vision, max_hands: Number(e.target.value) } })} />
          </Paper>
        </Grid>

        <Grid item xs={12} md={6}>
          <Paper sx={{ p: 2 }}>
            <Typography variant="h6" gutterBottom>Recognition</Typography>
            <Typography gutterBottom>Static Threshold: {config.recognition.static_threshold}</Typography>
            <Slider min={0} max={1} step={0.05} value={config.recognition.static_threshold}
              onChange={(_, v) => setConfig({ ...config, recognition: { ...config.recognition, static_threshold: v as number } })} />
            <Typography gutterBottom>Dynamic Threshold: {config.recognition.dynamic_threshold}</Typography>
            <Slider min={0} max={1} step={0.05} value={config.recognition.dynamic_threshold}
              onChange={(_, v) => setConfig({ ...config, recognition: { ...config.recognition, dynamic_threshold: v as number } })} />
            <Typography gutterBottom>Smoothing Factor: {config.recognition.smoothing_factor}</Typography>
            <Slider min={0} max={1} step={0.05} value={config.recognition.smoothing_factor}
              onChange={(_, v) => setConfig({ ...config, recognition: { ...config.recognition, smoothing_factor: v as number } })} />
          </Paper>
        </Grid>

        <Grid item xs={12} md={6}>
          <Paper sx={{ p: 2 }}>
            <Typography variant="h6" gutterBottom>Storage & Performance</Typography>
            <TextField label="Max Samples / Gesture" type="number" size="small" fullWidth sx={{ mb: 2 }}
              value={config.storage.max_samples_per_gesture}
              onChange={(e) => setConfig({ ...config, storage: { ...config.storage, max_samples_per_gesture: Number(e.target.value) } })} />
            <TextField label="Target FPS" type="number" size="small" fullWidth sx={{ mb: 2 }}
              value={config.performance.target_fps}
              onChange={(e) => setConfig({ ...config, performance: { ...config.performance, target_fps: Number(e.target.value) } })} />
            <TextField label="Log Level" size="small" fullWidth
              value={config.performance.log_level}
              onChange={(e) => setConfig({ ...config, performance: { ...config.performance, log_level: e.target.value } })} />
          </Paper>
        </Grid>
      </Grid>
    </Box>
  );
}
