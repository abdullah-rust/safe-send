import { useState, useRef, useEffect } from "react";
import { useLocation, useNavigate } from "react-router-dom";
import styles from "./css/EmailVerify.module.css";
import bgimg from "../assets/svg/code.svg";
import { invoke } from "@tauri-apps/api/core";

export default function EmailVerify() {
  const location = useLocation();
  const navigate = useNavigate();

  // @ts-ignore
  const state = location.state as { email: string; types: string };

  const [code, setCode] = useState(Array(6).fill(""));
  const [loading, setLoading] = useState(false); // <-- 🔄 loading state
  const inputs = useRef<HTMLInputElement[]>([]);

  const token = localStorage.getItem("jwt_token");

  useEffect(() => {
    if (token) {
      navigate("/", { replace: true });
    }
  });

  const handleInput = (
    index: number,
    e: React.ChangeEvent<HTMLInputElement>
  ) => {
    const value = e.target.value.replace(/\D/g, "");
    if (value.length > 1) return;

    const updated = [...code];
    updated[index] = value;
    setCode(updated);

    if (value && index < 5) {
      inputs.current[index + 1]?.focus();
    }
  };

  const handleKeyDown = (
    index: number,
    e: React.KeyboardEvent<HTMLInputElement>
  ) => {
    if (e.key === "Backspace") {
      const updated = [...code];
      if (updated[index]) {
        updated[index] = "";
        setCode(updated);
      } else if (index > 0) {
        inputs.current[index - 1]?.focus();
      }
    }
  };

  const handleSubmit = async () => {
    const finalCode = code.join("");
    setLoading(true); // <-- 🟡 Start loading

    try {
      let res = await invoke<string>("auth_verify_email", {
        data: {
          email: state.email,
          code: finalCode,
        },
        types: state.types,
      });

      if (typeof res === "string") {
        if (res.startsWith("eyJ")) {
          console.log("Login success! JWT Token:", res);
          localStorage.setItem("jwt_token", res);
          navigate("/", { replace: true });
        } else {
          console.warn("Server error:", res);
          alert(res);
        }
      }
    } catch (e) {
      alert(e);
    } finally {
      setLoading(false); // 🔵 Stop loading
    }
  };

  return (
    <main className={styles.container}>
      <img src={bgimg} alt="svg" className={styles.bgImage} />
      <div className={styles.card}>
        <h2 className={styles.title}>Enter 6-Digit Code From Email</h2>
        <div className={styles.otpBox}>
          {[0, 1, 2, 3, 4, 5].map((i) => (
            <input
              key={i}
              ref={(el) => (inputs.current[i] = el!)}
              className={styles.otpInput}
              type="text"
              maxLength={1}
              value={code[i]}
              onChange={(e) => handleInput(i, e)}
              onKeyDown={(e) => handleKeyDown(i, e)}
              disabled={loading} // 🔒 disable inputs during loading
            />
          ))}
        </div>
        <button
          className={styles.submitBtn}
          onClick={handleSubmit}
          disabled={loading} // 🔒 disable button during loading
        >
          {loading ? "Verifying..." : "Verify"} {/* 🔁 dynamic text */}
        </button>
      </div>
    </main>
  );
}
