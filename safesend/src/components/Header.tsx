import styles from "./css/header.module.css";
export default function Header() {
  // const [profile, setProfile] = useState(boyIcon);
  return (
    <header className={styles.header}>
      <h1 className={styles.h1}>SafeSend</h1>
      <img
        src={notificationIcon}
        alt="notification icon "
        className={styles.notiIcon}
      />

      <img src={boyIcon} alt="profile icon" className={styles.profile} />
    </header>
  );
}

// import icons

import boyIcon from "../assets/icons/boy.png";
// import womenIcon from "../assets/icons/woman.png";
import notificationIcon from "../assets/icons/notification-icon .png";
// import { useState } from "react";
