import { StrictMode } from "react";
import ReactDOM from "react-dom/client";
import { RouterProvider, createRouter } from "@tanstack/react-router";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { QueryProviderContext } from "./providers/QueryProvider";
import "./i18n";
// import "./i18next";

import "./assets/main.css";

// Import the generated route tree
import { routeTree } from "./routeTree.gen";

import { ThemeProvider, useTheme } from "./providers/ThemeProvider";
import { cn } from "./utils/tailwindUtils";
import { AppWindowProvider } from "./providers/AppWindowProvider";
import { LayoutProvider } from "./providers/LayoutProvider";
import { MetadataProvider } from "./providers/MetadataProvider";
import { ObjectProvider } from "./providers/ObjectProvider";
import { ConnectionProvider } from "./providers/ConnectionProvider";

// Create a new router instance
const router = createRouter({ routeTree });

// Register the router instance for type safety
declare module "@tanstack/react-router" {
  interface Register {
    router: typeof router;
  }
}

const queryClient = new QueryClient();

// Render the app
const rootElement = document.getElementById("root")!;
if (!rootElement.innerHTML) {
  const root = ReactDOM.createRoot(rootElement);
  root.render(
    <StrictMode>
      <AppWindowProvider>
        <ConnectionProvider>
          <QueryClientProvider client={queryClient}>
            <ThemeProvider>
              <LayoutProvider>
                <QueryProviderContext>
                  <ObjectProvider>
                    <MetadataProvider>
                      <App/>
                    </MetadataProvider>
                  </ObjectProvider>
                </QueryProviderContext>
              </LayoutProvider>
            </ThemeProvider>
          </QueryClientProvider>
        </ConnectionProvider>
      </AppWindowProvider>
    </StrictMode>
  );
}

function App() {
  const { /*isDark*/ } = useTheme();

  return (
    <main
      id="inner-root"
      className={cn("h-screen w-full", {
        //dark: isDark,
      })}
    >
      <RouterProvider router={router} />
    </main>
  );
}
