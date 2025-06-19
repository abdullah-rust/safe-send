import { lazy, Suspense } from "react";

import Loading from "../components/Loading";
const LazyChatContent = lazy(() => import("./main conatant/ChatContant"));

export default function Chat() {
  return (
    <main style={{ height: "100vh" }}>
      <Suspense fallback={<Loading />}>
        <LazyChatContent />
      </Suspense>
    </main>
  );
}
