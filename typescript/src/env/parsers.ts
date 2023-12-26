import { removeUndefined } from '#/utilities.ts'

/**
 * parse space-separated items of <profile>.<key>[.<chain_id]=<endpoint>into a mapping
 */
export function parseSpaceSeparatedProfiles(input?: string) {
  if (!input) return {}
  return input.split(' ').reduce((accumulator, pair) => {
    const [_key, value] = pair.split('=') as [string, string]
    const [profile, key, chain_id] = _key.split('.') as [string, string, string | undefined]
    return {
      ...accumulator,
      [profile]: {
        ...accumulator[profile],
        [key]: { ...accumulator[profile]?.[key], [`${chain_id}`]: value },
      },
    }
  }, {} as Record<string, Record<string, Record<string, string>>>)
}

/**
 * parse space-separated items of [<name>[:<chain_id>]=]<url> into a mapping
 * returns {Record<string, { url: string, chain_id?: string }>}
 */
export function parseSpaceSeparatedEndpoints(input?: string) {
  if (!input) return {}
  return removeUndefined(
    input.split(' ').reduce((accumulator, item) => {
      const [name, url] = item.split('=') as [string, string]
      const [name_, chain_id] = name.split(':') as [string, string | undefined]
      return { ...accumulator, [name_]: { url, chain_id } }
    }, {})
  ) as Record<string, { url: string; chain_id?: string }>
}

/**
 * parse space-separated pairs of <key>=<value> into a mapping
 */
export function parseSpaceSeparatedPairs(input?: string) {
  if (!input) return {}
  return input.split(' ').reduce((accumulator, pair) => {
    const [key, value] = pair.split('=') as [string, string]
    return { ...accumulator, [key]: value }
  }, {})
}
