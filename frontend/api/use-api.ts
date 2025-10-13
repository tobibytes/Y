import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query"

export type Method = "GET" | "POST" | "PUT" | "DELETE" | "PATCH"

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

// Body typing for different modes
export type BodyMode = "json" | "formData" | "text" | "raw" | "urlencoded"
export type FormDataScalar = string | Blob | File | number | boolean | Date
export type FormDataAppendable = FormDataScalar | FormDataScalar[]
export type FormDataObject = Record<string, FormDataAppendable>

function toFormDataValue(v: FormDataScalar): string | Blob {
  if (v instanceof Blob) return v
  if (v instanceof Date) return v.toISOString()
  // number/boolean/string
  return String(v)
}

function appendToFormData(fd: FormData, key: string, val: FormDataAppendable) {
  if (Array.isArray(val)) {
    for (const item of val) fd.append(key, toFormDataValue(item))
  } else {
    fd.append(key, toFormDataValue(val))
  }
}

function isFormDataObject(x: unknown): x is FormDataObject {
  return !!x && typeof x === "object" && !(x instanceof FormData)
}

function buildRequestInit<B>(
  method: Method,
  body: B | undefined,
  as: BodyMode,
  extraHeaders?: Record<string, string>
): RequestInit {
  const headers: Record<string, string> = {}
  let requestBody: BodyInit | undefined

  switch (as) {
    case "json": {
      if (body !== undefined) {
        headers["Content-Type"] = "application/json"
        requestBody = JSON.stringify(body)
      }
      break
    }
    case "formData": {
      if (body instanceof FormData) {
        requestBody = body
      } else if (isFormDataObject(body)) {
        const fd = new FormData()
        for (const key of Object.keys(body)) {
          const val = (body as FormDataObject)[key]
          if (val !== undefined && val !== null) appendToFormData(fd, key, val)
        }
        requestBody = fd
      }
      // Do not set Content-Type for FormData
      break
    }
    case "text": {
      headers["Content-Type"] = "text/plain"
      requestBody = body != null ? String(body as unknown as string) : ""
      break
    }
    case "urlencoded": {
      headers["Content-Type"] = "application/x-www-form-urlencoded"
      if (body instanceof URLSearchParams) {
        requestBody = body
      } else if (body && typeof body === "object") {
        const params = new URLSearchParams()
        for (const [k, v] of Object.entries(body as Record<string, unknown>)) {
          if (v === undefined || v === null) continue
          params.append(k, String(v))
        }
        requestBody = params.toString()
      } else if (typeof body === "string") {
        requestBody = body
      }
      break
    }
    case "raw":
    default: {
      requestBody = body as unknown as BodyInit
      break
    }
  }

  const finalHeaders = { ...headers, ...(extraHeaders ?? {}) }

  return {
    method,
    credentials: "include",
    headers: Object.keys(finalHeaders).length ? finalHeaders : undefined,
    body: requestBody,
  }
}

const useAPI = <
  Q extends Record<string, string> | undefined = undefined,
  B = unknown,
  R = unknown
>(
  route: Route<Q, B>,
  opts?: {
    query?: Q
    body?: B
    enabled?: boolean
    as?: BodyMode
    headers?: Record<string, string>
  }
) => {
  const queryClient = useQueryClient()
  const isGet = route.method === "GET"
  const url = buildUrl(route.url, opts?.query as Record<string, string> | undefined)
  const queryKeySuffix = opts?.query
    ? new URLSearchParams(opts.query as Record<string, string>).toString()
    : ""

  // Always call both hooks to keep hook order stable
  const query = useQuery<R, Error>({
    queryKey: [route.key, route.method, queryKeySuffix],
    queryFn: async () => {
      const res = await fetch(url, { method: "GET", credentials: "include" })
      if (!res.ok) throw new Error(`GET ${url} failed: ${res.status}`)
      return (await res.json()) as R
    },
    enabled: isGet && (opts?.enabled ?? true),
  })

  const mutation = useMutation<R, Error, B | undefined>({
    mutationKey: [route.key, route.method, queryKeySuffix],
    mutationFn: async (bodyArg: B | undefined) => {
      const init = buildRequestInit(
        route.method,
        bodyArg ?? opts?.body,
        (opts?.as ?? "json") as BodyMode,
        opts?.headers
      )
      const res = await fetch(url, init)
      if (!res.ok) throw new Error(`${route.method} ${url} failed: ${res.status}`)
      return (await res.json()) as R
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: [route.key] })
    },
  })

  return {
    isGet,
    query, // typed as R
    mutation, // typed as R
    mutate: (b?: B) => mutation.mutate(b ?? opts?.body),
    mutateAsync: (b?: B) => mutation.mutateAsync(b ?? opts?.body),
  } as const
}

export default useAPI