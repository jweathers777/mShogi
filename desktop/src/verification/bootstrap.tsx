import { StrictMode } from "react";
import type { Root } from "react-dom/client";
import { invoke } from "@tauri-apps/api/core";
import { ProductApp } from "../product/ProductApp";
import { SharedContractApp } from "./SharedContractApp";
import { SyntheticApp } from "./SyntheticApp";

export async function mountVerification(root: Root) {
  const mode = await invoke<string>("desktop_verification_mode");
  if (mode !== "chess" && mode !== "synthetic" && mode !== "shared-contract" && mode !== "product") throw new Error("Unknown verification session mode");
  root.render(<StrictMode>{mode === "synthetic" ? <SyntheticApp /> : mode === "shared-contract" ? <SharedContractApp /> : <ProductApp />}</StrictMode>);
}
