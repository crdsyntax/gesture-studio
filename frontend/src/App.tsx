import { Routes, Route } from "react-router-dom";
import { Box } from "@mui/material";
import Layout from "./components/Layout";
import DashboardPage from "./pages/DashboardPage";
import GesturesPage from "./pages/GesturesPage";
import TrainingPage from "./pages/TrainingPage";
import ActionsPage from "./pages/ActionsPage";
import SettingsPage from "./pages/SettingsPage";

function App() {
  return (
    <Box sx={{ display: "flex", height: "100vh" }}>
      <Layout>
        <Routes>
          <Route path="/" element={<DashboardPage />} />
          <Route path="/gestures" element={<GesturesPage />} />
          <Route path="/training" element={<TrainingPage />} />
          <Route path="/actions" element={<ActionsPage />} />
          <Route path="/settings" element={<SettingsPage />} />
        </Routes>
      </Layout>
    </Box>
  );
}

export default App;
