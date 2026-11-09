import {
  isRouteErrorResponse,
  Links,
  Meta,
  Outlet,
  Scripts,
  ScrollRestoration,
} from "react-router";

import type { Route } from "./+types/root";
import "./app.css";
import { AppProviders } from "./providers";

export const links: Route.LinksFunction = () => [
  { rel: "icon", href: "/favicon.ico?v=2", type: "image/x-icon" },
  { rel: "preconnect", href: "https://fonts.googleapis.com" },
  {
    rel: "preconnect",
    href: "https://fonts.gstatic.com",
    crossOrigin: "anonymous",
  },
  {
    rel: "stylesheet",
    href: "https://fonts.googleapis.com/css2?family=Inter:ital,opsz,wght@0,14..32,100..900;1,14..32,100..900&display=swap",
  },
];

export function Layout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="vi" suppressHydrationWarning>
      <head>
        <meta charSet="utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <Meta />
        <Links />
        {/* Inline script chặn FOUC: chạy trước khi React render */}
        <script
          dangerouslySetInnerHTML={{
            __html: `
              (function() {
                try {
                  var theme = localStorage.getItem('theme');
                  if (theme === 'dark' || (!theme && window.matchMedia('(prefers-color-scheme: dark)').matches)) {
                    document.documentElement.classList.add('dark');
                  } else {
                    document.documentElement.classList.add('light');
                  }
                  var locale = localStorage.getItem('merlink.locale');
                  if (locale === 'en' || locale === 'vi') {
                    document.documentElement.lang = locale;
                  }
                } catch (e) {}
              })();
            `,
          }}
        />
        <script
          dangerouslySetInnerHTML={{
            __html: `
              (function() {
                var retryKey = 'merlink:module-reload-at';
                var moduleFailure = /Importing a module script failed|Failed to fetch dynamically imported module|Outdated Optimize Dep/i;
                function showStartupFailure() {
                  var loading = document.querySelector('[data-chat-startup-loading]');
                  if (!loading || loading.querySelector('[data-startup-retry]')) return;
                  var english = document.documentElement.lang === 'en';
                  var title = loading.querySelector('h1');
                  var status = loading.querySelector('p');
                  if (title) title.textContent = english ? 'Merlink could not load' : 'Merlink không tải được';
                  if (status) {
                    status.textContent = english ? 'The app did not finish starting.' : 'Ứng dụng chưa khởi động xong.';
                    status.classList.remove('animate-pulse');
                  }
                  var button = document.createElement('button');
                  button.type = 'button';
                  button.dataset.startupRetry = '';
                  button.textContent = english ? 'Reload' : 'Tải lại';
                  button.className = 'mt-5 rounded-xl bg-accent px-4 py-2 font-semibold text-white';
                  button.onclick = function() {
                    try { sessionStorage.removeItem(retryKey); } catch (_) {}
                    window.location.reload();
                  };
                  loading.appendChild(button);
                }
                function handleModuleFailure(message) {
                  if (!moduleFailure.test(message || '')) return;
                  var lastReload = 0;
                  try { lastReload = Number(sessionStorage.getItem(retryKey)) || 0; } catch (_) {}
                  if (Date.now() - lastReload > 60000) {
                    try { sessionStorage.setItem(retryKey, String(Date.now())); } catch (_) {}
                    window.setTimeout(function() { window.location.reload(); }, 800);
                  } else {
                    showStartupFailure();
                  }
                }
                window.addEventListener('unhandledrejection', function(event) {
                  handleModuleFailure(event.reason && (event.reason.message || String(event.reason)));
                });
                window.addEventListener('error', function(event) {
                  handleModuleFailure(event.message);
                });
                window.setTimeout(showStartupFailure, 20000);
              })();
            `,
          }}
        />
      </head>
      <body>
        {children}
        <ScrollRestoration />
        <Scripts />
      </body>
    </html>
  );
}

export default function App() {
  return (
    <AppProviders>
      <Outlet />
    </AppProviders>
  );
}

export function ErrorBoundary({ error }: Route.ErrorBoundaryProps) {
  let message = "Oops!";
  let details = "An unexpected error occurred.";
  let stack: string | undefined;

  if (isRouteErrorResponse(error)) {
    message = error.status === 404 ? "404" : "Error";
    details =
      error.status === 404
        ? "The requested page could not be found."
        : error.statusText || details;
  } else if (import.meta.env.DEV && error && error instanceof Error) {
    details = error.message;
    stack = error.stack;
  }

  return (
    <main className="pt-16 p-4 container mx-auto">
      <h1>{message}</h1>
      <p>{details}</p>
      {stack && (
        <pre className="w-full p-4 overflow-x-auto">
          <code>{stack}</code>
        </pre>
      )}
    </main>
  );
}
