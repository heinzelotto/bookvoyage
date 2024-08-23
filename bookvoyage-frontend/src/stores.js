import { persist, createLocalStorage } from "@macfja/svelte-persistent-store";
import { writable } from "svelte/store";

export const user = persist(writable({}), createLocalStorage(), "user");
export const token = persist(writable({}), createLocalStorage(), "token");
export const isAuthenticated = persist(writable(false), createLocalStorage(), "isAuthenticated");