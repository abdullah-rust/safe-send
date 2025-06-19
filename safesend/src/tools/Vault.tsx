import { lazy, Suspense } from "react";

import Loading from "../components/Loading";

const LazyVaultContent = lazy(() => import("./main conatant/VaultContant"));
export default function Vault() {
  return (
    <main style={{ height: "100vh" }}>
      <Suspense fallback={<Loading />}>
        <LazyVaultContent />
      </Suspense>
    </main>
  );
}
