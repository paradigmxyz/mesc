export function raise(error: unknown): never {
  throw typeof error === 'string' ? new Error(error) : error
}

// removed properties with undefined values from object
export function removeUndefined<T>(object: T): T {
  return JSON.parse(JSON.stringify(object)) as T
}
