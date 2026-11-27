import { StrictMode } from "react";
import type { Root } from "react-dom/client";
import { invoke } from "@tauri-apps/api/core";
import { ProductApp } from "../product/ProductApp";
import { SyntheticApp } from "./SyntheticApp";
import { TenjikuControls } from "./TenjikuControls";

export async function mountVerification(root: Root) {
  const mode = await invoke<string>("desktop_verification_mode");
  if (mode !== "synthetic" && mode !== "product" && mode !== "tenjiku-contract") {
    throw new Error("Unknown verification session mode");
  }
  const controls = mode === "tenjiku-contract" ? TenjikuControls : undefined;
  root.render(
    <StrictMode>
      {mode === "synthetic" ? <SyntheticApp /> : <ProductApp verificationControls={controls} />}
    </StrictMode>,
  );
}
