import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query"

export type Method = "GET" | "POST" | "PUT" | "DELETE"

export interface Route<Q extends Record<string, string> | undefined = undefined, B = unknown | undefined> {
  key: string
  url: string
  method: Method
}

const API_BASE = process.env.NEXT_PUBLIC_BACKEND_URL as string

function buildUrl(path: string, query?: Record<string, string>) {
  const qs = query && Object.keys(query).length ? `?${new URLSearchParams(query).toString()}` : ""
  return `${API_BASE}${path}${qs}`
}

const useAPI = <Q extends Record<string, string> | undefined = undefined, B = unknown | undefined>(
  route: Route<Q, B>,
  opts?: { query?: Q; body?: B; enabled?: boolean }
) => {
  const queryClient = useQueryClient()
  const isGet = route.method === "GET"
  const url = buildUrl(route.url, opts?.query as Record<string, string> | undefined)

  // Always call both hooks to keep hook order stable
  const query = useQuery<unknown, Error>({
    queryKey: [route.key, route.method, opts?.query],
    queryFn: async () => {
      const res = await fetch(url, { method: "GET", credentials: "include" })
     if (!res.ok) throw new Error(`GET ${url} failed: ${res.status}`)
      return res.json()
    },
    enabled: isGet && (opts?.enabled ?? true),
  })

  const mutation = useMutation<unknown, Error, B | undefined>({
    mutationKey: [route.key, route.method, opts?.query],
    mutationFn: async (body: B | undefined) => {
      const res = await fetch(url, {
        method: route.method,
        headers: { "Content-Type": "application/json" },
        credentials: "include",
        body: body != null ? JSON.stringify(body) : undefined,
      })
      if (!res.ok) throw new Error(`${route.method} ${url} failed: ${res.status}`)
      return res.json()
    },
    onSuccess: () => {
      // Invalidate queries tied to this key; adjust as needed
      queryClient.invalidateQueries({ queryKey: [route.key] })
    },
  })

  return {
    isGet,
    query,                  // use when route.method === "GET"
    mutation,               // use when route.method !== "GET"
    mutate: (b?: B) => mutation.mutate(b ?? opts?.body),
  } as const
}

export default useAPI