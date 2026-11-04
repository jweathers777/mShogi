import { StrictMode } from "react";
import type { Root } from "react-dom/client";
import { invoke } from "@tauri-apps/api/core";
import { ProductApp } from "../product/ProductApp";
import { SyntheticApp } from "./SyntheticApp";
import { TenjikuControls } from "./TenjikuControls";

export async function mountVerification(root: Root) {
  const mode = await invoke<string>("desktop_verification_mode");
  if (mode !== "chess" && mode !== "synthetic" && mode !== "shared-contract" && mode !== "product" && mode !== "tenjiku-contract") throw new Error("Unknown verification session mode");
  root.render(<StrictMode>{mode === "synthetic" ? <SyntheticApp /> : mode === "tenjiku-contract" ? <ProductApp verification verificationControls={TenjikuControls} /> : <ProductApp />}</StrictMode>);
}
