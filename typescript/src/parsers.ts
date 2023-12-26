import fs from 'node:fs'
import * as v from 'valibot'
import { raise, removeUndefined } from '#/utilities.ts'
import { rpcConfigSchema } from '#/schemas/rpc-config.ts'
import type { RawMESCConfiguration } from '#/schemas/mesc-configuration.ts'

/** @see https://github.com/paradigmxyz/mesc/blob/main/SPECIFICATION.md#environment-setup */
export function parseMescVariables({
  MESC_MODE,
  MESC_PATH,
  MESC_ENV,
}: Pick<RawMESCConfiguration, 'MESC_MODE' | 'MESC_PATH' | 'MESC_ENV'>) {
  if (MESC_MODE.length > 0 && !['PATH', 'ENV'].includes(MESC_MODE)) {
    throw new Error('MESC_MODE must be set to PATH or ENV')
  }

  if (MESC_MODE === 'PATH') {
    const mescFileExists = fs.existsSync(MESC_PATH)
    if (!mescFileExists) raise('MESC_MODE is set to PATH but file set in MESC_PATH does not exist')
    const rawRpcConfig = fs.readFileSync(MESC_PATH, { encoding: 'utf-8' })
    return v.parse(rpcConfigSchema, JSON.parse(rawRpcConfig), { abortEarly: false, abortPipeEarly: false })
  }
  if (MESC_MODE === 'ENV') {
    const rawRpcConfig = MESC_ENV?.length ? MESC_ENV : raise('MESC_MODE is set to ENV but MESC_ENV is not set or empty')
    return v.parse(rpcConfigSchema, JSON.parse(rawRpcConfig), { abortEarly: false, abortPipeEarly: false })
  }
  raise('MESC_MODE is not enabled')
}

/**
 * parse space-separated items of <profile>.<key>[.<chain_id]=<endpoint>into a mapping
 */
export function parseSpaceSeparatedProfiles(input?: string) {
  if (!input) return false
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
  if (!input) return false
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
  if (!input) return false
  return input.split(' ').reduce((accumulator, pair) => {
    const [key, value] = pair.split('=') as [string, string]
    return { ...accumulator, [key]: value }
  }, {})
}

/**
 * parse JSON string into a JSON object
 */
export function parseStringJSON(input?: string) {
  if (!input) return false
  return JSON.parse(input) as Record<string, unknown>
}
