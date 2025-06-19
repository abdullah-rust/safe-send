import { motion } from "framer-motion"; // sahi motion lib
import { invoke } from "@tauri-apps/api/core";



export default function ChatContent() {
  const fetchProduct = async () => {
    try {
    const resRaw = await invoke("auth_signup", {
  data: {
    name: "Abdullah",
    age: 18343454,
    gender: "Male",
    email: "abdullahriaz381a@gmail.com",
    password: "Abdullah226622"
  }
});

console.log(resRaw);
// 👈 parse the string


    } catch (e) {
      console.error("🔥 Exception:", e);
    }
  };

  return (
    <motion.div
      initial={{ opacity: 0, x: 20 }}
      animate={{ opacity: 1, x: 0 }}
      transition={{ duration: 0.5 }}
      style={{ marginTop: "60px" }}
    >
      <h1>Chat Content</h1>
      <button onClick={fetchProduct}>Click</button>
    </motion.div>
  );
}
