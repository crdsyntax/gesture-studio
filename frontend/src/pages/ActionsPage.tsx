import { useEffect, useState } from "react";
import {
  Typography, Box, Paper, Button, List, ListItem, ListItemText,
  IconButton, Dialog, DialogTitle, DialogContent, DialogActions,
  TextField, Select, MenuItem, FormControl, InputLabel, Chip,
  Alert,
} from "@mui/material";
import DeleteIcon from "@mui/icons-material/Delete";
import AddIcon from "@mui/icons-material/Add";
import PlayArrowIcon from "@mui/icons-material/PlayArrow";
import { invoke } from "@tauri-apps/api/core";

interface GestureInfo {
  id: string;
  name: string;
  gesture_type: string;
  created_at: string;
}

interface ActionInfo {
  id: string;
  gesture_id: string;
  action_type: string;
  payload: string;
  enabled: boolean;
}

const ACTION_TYPES = [
  "OpenApp", "ExecuteCommand", "OpenUrl", "ChangeVolume",
  "MediaControl", "LockWorkstation", "SimulateKeyboard",
  "SimulateMouse", "HttpRequest", "PowerShell", "Bash", "TauriEvent",
];

export default function ActionsPage() {
  const [gestures, setGestures] = useState<GestureInfo[]>([]);
  const [selectedGesture, setSelectedGesture] = useState<string>("");
  const [actions, setActions] = useState<ActionInfo[]>([]);
  const [open, setOpen] = useState(false);
  const [newType, setNewType] = useState("OpenApp");
  const [newPayload, setNewPayload] = useState("");
  const [error, setError] = useState("");
  const [execResult, setExecResult] = useState<string>("");

  const loadGestures = () => {
    invoke<GestureInfo[]>("list_gestures").then(setGestures).catch(console.error);
  };

  const loadActions = (gestureId: string) => {
    invoke<ActionInfo[]>("list_actions", { gestureId })
      .then(setActions)
      .catch(console.error);
  };

  useEffect(() => { loadGestures(); }, []);

  useEffect(() => {
    if (selectedGesture) loadActions(selectedGesture);
    else setActions([]);
  }, [selectedGesture]);

  const handleCreate = async () => {
    setError("");
    try {
      await invoke("create_action", {
        gestureId: selectedGesture,
        actionType: newType,
        payload: newPayload,
      });
      setOpen(false);
      setNewPayload("");
      loadActions(selectedGesture);
    } catch (e) {
      setError(String(e));
    }
  };

  const handleDelete = async (id: string) => {
    await invoke("delete_action", { actionId: id });
    loadActions(selectedGesture);
  };

  const handleExecute = async (id: string) => {
    try {
      const result = await invoke<{ success: boolean; message: string }>("execute_action", { actionId: id });
      setExecResult(result.message);
    } catch (e) {
      setError(String(e));
    }
  };

  return (
    <Box>
      <Typography variant="h4" gutterBottom>Actions</Typography>

      {error && <Alert severity="error" sx={{ mb: 2 }}>{error}</Alert>}
      {execResult && <Alert severity="success" sx={{ mb: 2 }} onClose={() => setExecResult("")}>{execResult}</Alert>}

      <Paper sx={{ p: 2, mb: 2 }}>
        <Typography variant="h6" gutterBottom>Select Gesture</Typography>
        {gestures.length === 0 ? (
          <Typography color="text.secondary">No gestures yet.</Typography>
        ) : (
          <Box sx={{ display: "flex", gap: 1, flexWrap: "wrap" }}>
            {gestures.map((g) => (
              <Chip
                key={g.id}
                label={g.name}
                color={selectedGesture === g.id ? "primary" : "default"}
                onClick={() => setSelectedGesture(g.id)}
              />
            ))}
          </Box>
        )}
      </Paper>

      {selectedGesture && (
        <Paper sx={{ p: 2 }}>
          <Box sx={{ display: "flex", justifyContent: "space-between", mb: 2 }}>
            <Typography variant="h6">Actions</Typography>
            <Button startIcon={<AddIcon />} variant="contained" onClick={() => setOpen(true)}>
              Add Action
            </Button>
          </Box>

          {actions.length === 0 ? (
            <Typography color="text.secondary">No actions assigned to this gesture.</Typography>
          ) : (
            <List>
              {actions.map((a) => (
                <ListItem key={a.id}>
                  <ListItemText
                    primary={a.action_type}
                    secondary={a.payload}
                  />
                  <IconButton onClick={() => handleExecute(a.id)}><PlayArrowIcon /></IconButton>
                  <IconButton onClick={() => handleDelete(a.id)}><DeleteIcon /></IconButton>
                </ListItem>
              ))}
            </List>
          )}
        </Paper>
      )}

      <Dialog open={open} onClose={() => setOpen(false)} maxWidth="sm" fullWidth>
        <DialogTitle>Add Action</DialogTitle>
        <DialogContent sx={{ display: "flex", flexDirection: "column", gap: 2, pt: 2 }}>
          <FormControl fullWidth>
            <InputLabel>Action Type</InputLabel>
            <Select value={newType} label="Action Type"
              onChange={(e) => setNewType(e.target.value)}>
              {ACTION_TYPES.map((t) => (
                <MenuItem key={t} value={t}>{t}</MenuItem>
              ))}
            </Select>
          </FormControl>
          <TextField label="Payload" fullWidth multiline rows={3}
            value={newPayload}
            onChange={(e) => setNewPayload(e.target.value)}
            placeholder={newType === "OpenApp" ? "C:\\Path\\to\\app.exe" :
              newType === "OpenUrl" ? "https://example.com" :
              "Command or configuration string"}
          />
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setOpen(false)}>Cancel</Button>
          <Button onClick={handleCreate} variant="contained">Create</Button>
        </DialogActions>
      </Dialog>
    </Box>
  );
}
