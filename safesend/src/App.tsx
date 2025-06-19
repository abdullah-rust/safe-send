import Home from "./Home";
import "./App.css";
import { Routes, Route, Navigate, useLocation } from "react-router-dom";
import { AnimatePresence } from "motion/react";
import LogIn from "./auth/Login";
import SignUp from "./auth/Signup";
import EmailVerify from "./auth/Emailcodeverify";
import Main from "./auth/Main";

function App() {
  const token = localStorage.getItem("jwt_token"); // token key match karo
  const location = useLocation(); // to support AnimatePresence

  return (
    <AnimatePresence mode="wait">
      <Routes location={location} key={location.pathname}>
        {/* ✅ Protected Route: if token, show Home. Otherwise redirect to /main */}
        <Route
          path="/"
          element={token ? <Home /> : <Navigate to="/main" replace />}
        />

        {/* Public Routes */}
        <Route path="/login" element={<LogIn />} />
        <Route path="/signup" element={<SignUp />} />
        <Route path="/verify" element={<EmailVerify />} />
        <Route path="/main" element={<Main />} />
      </Routes>
    </AnimatePresence>
  );
}

export default App;
