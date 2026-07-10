import { useEffect, useState } from "react";
import {
  Typography, Box, Paper, Button, List, ListItem, ListItemButton,
  ListItemText, Stepper, Step, StepLabel, TextField, Dialog,
  DialogTitle, DialogContent, DialogActions, Select, MenuItem,
  FormControl, InputLabel, LinearProgress, Alert, Grid,
} from "@mui/material";
import { invoke } from "@tauri-apps/api/core";
import CameraPreview from "../components/CameraPreview";

interface GestureInfo {
  id: string;
  name: string;
  gesture_type: string;
  created_at: string;
}

const STEPS = ["Select Gesture", "Capture Samples", "Finish"];

export default function TrainingPage() {
  const [gestures, setGestures] = useState<GestureInfo[]>([]);
  const [selectedGesture, setSelectedGesture] = useState<string>("");
  const [activeStep, setActiveStep] = useState(0);
  const [sampleCount, setSampleCount] = useState(0);
  const [finishOpen, setFinishOpen] = useState(false);
  const [newName, setNewName] = useState("");
  const [newType, setNewType] = useState("static");
  const [error, setError] = useState("");

  useEffect(() => {
    invoke<GestureInfo[]>("list_gestures").then(setGestures).catch(console.error);
  }, []);

  const handleStartTraining = async () => {
    if (!selectedGesture) return;
    setError("");
    try {
      await invoke("start_training", { gestureId: selectedGesture });
      setSampleCount(0);
      setActiveStep(1);
    } catch (e) {
      setError(String(e));
    }
  };

  const handleCaptureSample = async () => {
    try {
      const count = await invoke<number>("capture_training_sample");
      setSampleCount(count);
    } catch (e) {
      setError(String(e));
    }
  };

  const handleFinishOpen = () => {
    setFinishOpen(true);
  };

  const handleFinish = async () => {
    try {
      await invoke("finish_training", { name: newName, gestureType: newType });
      setFinishOpen(false);
      setActiveStep(0);
      setSelectedGesture("");
      setNewName("");
      const updated = await invoke<GestureInfo[]>("list_gestures");
      setGestures(updated);
      setActiveStep(2);
      setTimeout(() => setActiveStep(0), 1500);
    } catch (e) {
      setError(String(e));
    }
  };

  return (
    <Box>
      <Typography variant="h4" gutterBottom>Training</Typography>

      <Stepper activeStep={activeStep} sx={{ mb: 3 }}>
        {STEPS.map((label) => (
          <Step key={label}><StepLabel>{label}</StepLabel></Step>
        ))}
      </Stepper>

      {error && <Alert severity="error" sx={{ mb: 2 }}>{error}</Alert>}

      <Grid container spacing={2}>
        {(activeStep === 0) && (
          <Grid item xs={12}>
            <Paper sx={{ p: 3 }}>
              <Typography variant="h6" gutterBottom>Select a gesture to train</Typography>
              {gestures.length === 0 ? (
                <Typography color="text.secondary">
                  No gestures yet. Create one in the Gestures page first.
                </Typography>
              ) : (
                <List>
                  {gestures.map((g) => (
                    <ListItem key={g.id} disablePadding>
                      <ListItemButton
                        selected={selectedGesture === g.id}
                        onClick={() => setSelectedGesture(g.id)}
                      >
                        <ListItemText primary={g.name} secondary={g.gesture_type} />
                      </ListItemButton>
                    </ListItem>
                  ))}
                </List>
              )}
              <Button
                variant="contained"
                onClick={handleStartTraining}
                disabled={!selectedGesture}
                sx={{ mt: 2 }}
              >
                Start Training
              </Button>
            </Paper>
          </Grid>
        )}

        {activeStep === 1 && (
          <>
            <Grid item xs={12} md={7}>
              <CameraPreview height={360} />
            </Grid>
            <Grid item xs={12} md={5}>
              <Paper sx={{ p: 3, height: "100%" }}>
                <Typography variant="h6" gutterBottom>Capture samples</Typography>
                <Typography variant="body2" color="text.secondary" sx={{ mb: 2 }}>
                  Show your hand to the camera and click "Capture Sample". Repeat for more samples.
                </Typography>
                <LinearProgress
                  variant="determinate"
                  value={Math.min((sampleCount / 10) * 100, 100)}
                  sx={{ mb: 2 }}
                />
                <Typography variant="h3" sx={{ textAlign: "center", mb: 2 }}>
                  {sampleCount} <Typography variant="body1" component="span" color="text.secondary">/ 10</Typography>
                </Typography>
                <Box sx={{ display: "flex", flexDirection: "column", gap: 2 }}>
                  <Button variant="contained" size="large" onClick={handleCaptureSample}>
                    Capture Sample
                  </Button>
                  <Button
                    variant="outlined"
                    onClick={handleFinishOpen}
                    disabled={sampleCount < 3}
                  >
                    Finish Training
                  </Button>
                </Box>
              </Paper>
            </Grid>
          </>
        )}
      </Grid>

      {activeStep === 2 && (
        <Alert severity="success" sx={{ mt: 2 }}>Template saved successfully!</Alert>
      )}

      <Dialog open={finishOpen} onClose={() => setFinishOpen(false)}>
        <DialogTitle>Finish Training</DialogTitle>
        <DialogContent sx={{ display: "flex", flexDirection: "column", gap: 2, pt: 2 }}>
          <TextField label="Gesture Name" fullWidth value={newName}
            onChange={(e) => setNewName(e.target.value)} />
          <FormControl fullWidth>
            <InputLabel>Type</InputLabel>
            <Select value={newType} label="Type"
              onChange={(e) => setNewType(e.target.value)}>
              <MenuItem value="static">Static</MenuItem>
              <MenuItem value="dynamic">Dynamic</MenuItem>
            </Select>
          </FormControl>
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setFinishOpen(false)}>Cancel</Button>
          <Button onClick={handleFinish} variant="contained" disabled={!newName}>
            Save Template
          </Button>
        </DialogActions>
      </Dialog>
    </Box>
  );
}
