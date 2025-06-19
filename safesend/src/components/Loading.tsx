// src/components/Loading.tsx
import styles from "./css/Loading.module.css";
import logo from "../../app-icon.png"; // ✅ tu yahan apna image daal sakta hai

export default function Loading() {
  return (
    <div className={styles.container}>
      <img src={logo} alt="Logo" className={styles.logo} />
      <div className={styles.text}>Loading...</div>
    </div>
  );
}
