import styles from "./css/main.module.css";
import { motion } from "motion/react";
import { useNavigate } from "react-router-dom";
import mainScreen from "../assets/svg/Android.svg";

export default function Main() {
  const navigate = useNavigate();
  return (
    <motion.main
      className={styles.container}
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      transition={{ duration: 1 }}
      exit={{ opacity: 0, transition: { duration: 0.3 } }}
    >
      <img src={mainScreen} alt="photo" className={styles.bgImage} />

      <div className={styles.box}>
        <h2>Welcome To SafeSend</h2>
        <p>
          SafeSend keeps your data completely secure. Our app uses end-to-end
          encryption to ensure that your chats and files remain private at all
          times. With modern technology and advanced security protocols, your
          data is always under your control.
        </p>
      </div>

      <div className={styles.button_div}>
        <button className={styles.bt1} onClick={() => navigate("/signup")}>
          Sign Up
        </button>
        <button className={styles.bt2} onClick={() => navigate("/login")}>
          Log In
        </button>
      </div>
    </motion.main>
  );
}
