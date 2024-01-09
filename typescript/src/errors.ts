export class MescDisabledError extends Error {
  constructor(public message = 'MESCs are disabled') {
    super(message)
  }
}

export class MissingEndpointError extends Error {
  constructor(public message = 'Missing endpoint') {
    super(message)
  }
}

export class InvalidOverrideError extends Error {
  constructor(public message = 'Invalid override') {
    super(message)
  }
}
