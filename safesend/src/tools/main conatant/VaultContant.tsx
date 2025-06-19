// src/tools/ChatContent.tsx
import { motion } from "motion/react";

export default function VaultContent() {
  return (
    <motion.div
      initial={{ opacity: 0, x: 20 }}
      animate={{ opacity: 1, x: 0 }}
      transition={{ duration: 0.5 }}
      style={{ marginTop: "60px" }}
    >
      <h1>This is vault</h1>
    </motion.div>
  );
}
