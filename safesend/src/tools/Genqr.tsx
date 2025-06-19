import { lazy, Suspense } from "react";

import Loading from "../components/Loading";

const LazyGenqrContent = lazy(() => import("./main conatant/GenqrContant"));

export default function Genqr() {
  return (
    <main style={{ height: "100vh" }}>
      <Suspense fallback={<Loading />}>
        <LazyGenqrContent />
      </Suspense>
    </main>
  );
}
