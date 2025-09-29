
export type Method = "GET" | "POST" | "PUT" | "DELETE"

 interface RouteDef {
  key: string
  url: string         
  method: Method
}

const ROUTE = {
  GET_OAUTH_URL: {
    key: "oauth-url",
    url: "/auth/google/url",
    method: "GET",
  } satisfies RouteDef,
} as const

export default ROUTE
export type { RouteDef }