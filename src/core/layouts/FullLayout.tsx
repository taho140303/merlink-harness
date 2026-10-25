import { Outlet } from "react-router";

export default function FullLayout() {
  return (
    <div className="flex min-h-screen flex-col bg-background text-foreground">
      <header className="border-b px-6 py-4">
        <h1 className="text-lg font-semibold tracking-tight">Merlink Harness</h1>
      </header>
      <main className="flex-1 p-6">
        <Outlet />
      </main>
      <footer className="border-t px-6 py-4 text-center text-sm text-muted-foreground">
        © {new Date().getFullYear()} Merlink Harness. All rights reserved.
      </footer>
    </div>
  );
}
