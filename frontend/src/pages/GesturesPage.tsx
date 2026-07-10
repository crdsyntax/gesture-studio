import { useEffect, useState } from "react";
import {
  Typography,
  Button,
  List,
  ListItem,
  ListItemText,
  IconButton,
  Dialog,
  DialogTitle,
  DialogContent,
  TextField,
  DialogActions,
} from "@mui/material";
import DeleteIcon from "@mui/icons-material/Delete";
import AddIcon from "@mui/icons-material/Add";
import { invoke } from "@tauri-apps/api/core";

interface GestureInfo {
  id: string;
  name: string;
  gesture_type: string;
  created_at: string;
}

export default function GesturesPage() {
  const [gestures, setGestures] = useState<GestureInfo[]>([]);
  const [open, setOpen] = useState(false);
  const [newName, setNewName] = useState("");

  const load = () => {
    invoke<GestureInfo[]>("list_gestures").then(setGestures).catch(console.error);
  };

  useEffect(load, []);

  const handleCreate = async () => {
    await invoke("create_gesture", { name: newName, gestureType: "static" });
    setOpen(false);
    setNewName("");
    load();
  };

  const handleDelete = async (id: string) => {
    await invoke("delete_gesture", { gestureId: id });
    load();
  };

  return (
    <>
      <Typography variant="h4" gutterBottom>
        Gestures
        <Button
          variant="contained"
          startIcon={<AddIcon />}
          onClick={() => setOpen(true)}
          sx={{ ml: 2 }}
        >
          New
        </Button>
      </Typography>

      <List>
        {gestures.map((g) => (
          <ListItem
            key={g.id}
            secondaryAction={
              <IconButton onClick={() => handleDelete(g.id)}>
                <DeleteIcon />
              </IconButton>
            }
          >
            <ListItemText
              primary={g.name}
              secondary={`${g.gesture_type} — ${g.created_at}`}
            />
          </ListItem>
        ))}
      </List>

      <Dialog open={open} onClose={() => setOpen(false)}>
        <DialogTitle>Create Gesture</DialogTitle>
        <DialogContent>
          <TextField
            autoFocus
            label="Name"
            fullWidth
            value={newName}
            onChange={(e) => setNewName(e.target.value)}
          />
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setOpen(false)}>Cancel</Button>
          <Button onClick={handleCreate} variant="contained">
            Create
          </Button>
        </DialogActions>
      </Dialog>
    </>
  );
}
