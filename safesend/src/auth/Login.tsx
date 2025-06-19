import styles from "./css/login.module.css";
import SvgBg from "../assets/svg/login.svg";
import { motion } from "motion/react";
import { useState, useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { invoke } from "@tauri-apps/api/core";

export default function LogIn() {
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [errormsg, setErrormsg] = useState("");
  const [loading, setLoading] = useState(false);
  const navigate = useNavigate();

  const token = localStorage.getItem("jwt_token");

  useEffect(() => {
    if (token) {
      navigate("/", { replace: true });
    }
  });

  // handle form
  async function handleSubmit(e: React.FormEvent<HTMLFormElement>) {
    e.preventDefault();

    if (!isFormValid()) {
      setErrormsg("⚠️ Please fill in all fields.");
      return;
    }

        try{
          setLoading(!loading)
           
          let res=await invoke<string>("auth_login",{
            data:{
              email:email,
              password:password
            }
          });

          if(res == "Verification OK, code sent to email"){
            navigate("/verify",{replace:true,state:{email:email,types:"login"}})
          }


        }catch(e){
          console.log(e);
          
        }

  
  }

  function isFormValid() {
    return email.trim() !== "" && password.trim() !== "";
  }

  return (
    <motion.main
      className={styles.container}
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      transition={{ duration: 0.5 }}
    >
      <img src={SvgBg} alt="svg" className={styles.bgImage} />
      <div className={styles.box}>
        <h1 className={styles.heading}>Welcome Back</h1>

        <form className={styles.form} onSubmit={handleSubmit}>
          <input
            type="email"
            placeholder="Email"
            required
            onChange={(e) => setEmail(e.target.value)}
          />
          <input
            type="password"
            placeholder="Password"
            required
            onChange={(e) => setPassword(e.target.value)}
          />

          <button type="submit" disabled={loading}>
            {loading ? "Logging in..." : "Log In"}
          </button>
        </form>

        {loading && <div className={styles.spinner}></div>}
      </div>

      <motion.div
        className={errormsg.length > 0 ? styles.Errormessage : ""}
        initial={{ y: -100 }}
        animate={{ y: 0 }}
        transition={{ duration: 0.5 }}
      >
        <h3>{errormsg}</h3>
      </motion.div>
    </motion.main>
  );
}
