"use client";
import React, { useEffect } from "react";
import { useRouter, useSearchParams } from "next/navigation";
import useAPI from "@/api/use-api";
import ROUTE from "@/api/route";

const Callback: React.FC = () => {
  const router = useRouter();
  const params = useSearchParams();

  const code = params.get("code") || "";
  const state = params.get("state") || "";

  const enabled = Boolean(code && state);

  const { query } = useAPI(ROUTE.GOOGLE_OAUTH_CALLBACK, {
    query: { code, state },
    enabled,
  });

  const { data, isLoading, error, isSuccess } = query as {
    data?: { ok?: boolean };
    isLoading: boolean;
    error: unknown;
    isSuccess: boolean;
  };

  useEffect(() => {
    if (isSuccess && data && data.ok !== false) {
      // Session cookie set by backend (HttpOnly, Secure when https). Nothing to store on client.
      router.replace("/");
    }
  }, [isSuccess, data, router]);

  if (!enabled) {
    return (
      <div className="p-6">
        Missing OAuth parameters. Please try signing in again.
      </div>
    );
  }

  if (isLoading) {
    return <div className="p-6">Signing you in…</div>;
  }

  if (error) {
    return <div className="p-6">Authentication failed.</div>;
  }

  return <div className="p-6">Finalizing sign-in…</div>;
};

export default Callback;
