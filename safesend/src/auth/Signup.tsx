import { motion } from "motion/react";
import styles from "./css/signup.module.css";
import SvgBg from "../assets/svg/signup.svg";
import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useNavigate } from "react-router-dom";

export default function SignUp() {
  const [name, setName] = useState("");
  const [age, setAge] = useState(0);
  const [gender, setGender] = useState("");
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

  async function handleSubmit(e: any) {
    e.preventDefault();
         if(isFormValid()){
          try{
             setLoading(!loading)
            let res= await invoke<string>("auth_signup",{
              data:{
                name:name,
                age:age,
                gender:gender,
                email:email,
                password:password
              }
            });

            if(res =="Verification OK, code sent to email"){
              navigate("/verify",{replace:true,state:{email:email,types:"signup"}})
            }
            setErrormsg(res);

            console.log(res);
            

          }catch(e){
            console.log(e);
            
          }
         }
  }

  function isFormValid() {
    return (
      name.trim() !== "" &&
      age > 0 &&
      gender.trim() !== "" &&
      email.trim() !== "" &&
      password.trim() !== ""
    );
  }

  const handleGenderChange = (e: any) => {
    const value = (e.target as HTMLSelectElement).value;
    setGender(value);
  };

  return (
    <motion.main
      className={styles.container}
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      transition={{ duration: 0.5 }}
    >
      <img src={SvgBg} alt="svg" className={styles.bgImage} />
      <div className={styles.box}>
        <h1 className={styles.heading}>Create Account</h1>
        <form className={styles.form} onSubmit={handleSubmit}>
          <input
            type="text"
            placeholder="Name"
            required
            onChange={(e) => setName(e.target.value)}
          />
          <input
            type="number"
            placeholder="Age"
            required
            onChange={(e) => setAge(parseInt(e.target.value))}
          />
          <select required onChange={handleGenderChange}>
            <option value="">Select Gender</option>
            <option value="male">Male</option>
            <option value="female">Female</option>
            <option value="other">Other</option>
          </select>
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
            {loading ? "Signing up..." : "Sign Up"}
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
