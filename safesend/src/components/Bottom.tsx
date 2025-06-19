// Bottom.tsx
import styles from "./css/Bottom.module.css";
import chatIcon from "../assets/icons/icons8-chat-100.png";
import vaultIcon from "../assets/icons/vault-icon.png";
import qrIcon from "../assets/icons/qr-icon.png";

type BottomProps = {
  onTabClick: (tab: string) => void;
  selectedTab: string;
};

export default function Bottom(props: BottomProps) {
  return (
    <main className={styles.bot}>
      <div className={styles.icon} onClick={() => props.onTabClick("chat")}>
        <img src={chatIcon} alt="chatIcon" />
        <p>Chat</p>
        {props.selectedTab === "chat" && <div className={styles.line}></div>}
      </div>

      <div className={styles.icon} onClick={() => props.onTabClick("vault")}>
        <img src={vaultIcon} alt="vaultIcon" />
        <p>Vault</p>
        {props.selectedTab === "vault" && <div className={styles.line}></div>}
      </div>

      <div className={styles.icon} onClick={() => props.onTabClick("qr")}>
        <img src={qrIcon} alt="qrIcon" />
        <p>QR Gen</p>
        {props.selectedTab === "qr" && <div className={styles.line}></div>}
      </div>
    </main>
  );
}
