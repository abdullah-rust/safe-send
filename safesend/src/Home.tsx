import Header from "./components/Header";
import Bottom from "./components/Bottom";
import { useState } from "react";
import Chat from "./tools/Chat";
import Vault from "./tools/Vault";
import Genqr from "./tools/Genqr";

export default function Home() {
  const [SelectedTab, setSelectedTab] = useState("chat");
  const handleTabClick = (tab: string) => {
    console.log("Selected tab:", tab);
    setSelectedTab(tab);
    // yahan tum tab set kar sakte ho ya routing, etc.
  };
  return (
    <main>
      <Header />
      {SelectedTab === "chat" && <Chat />}
      {SelectedTab === "vault" && <Vault />}
      {SelectedTab === "qr" && <Genqr />}
      <Bottom onTabClick={handleTabClick} selectedTab={SelectedTab} />
    </main>
  );
}
