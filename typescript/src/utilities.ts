export function raise(error: unknown): never {
  throw typeof error === 'string' ? new Error(error) : error
}

export const unixTimestamp = () => `${Math.floor(Date.now() / 1_000)}`

// removed properties with undefined values from object
export function removeUndefined<T>(object?: T): T {
  if (!object) return {} as T
  return (typeof object === 'string' ? JSON.parse(object) : JSON.parse(JSON.stringify(object))) as T
}
