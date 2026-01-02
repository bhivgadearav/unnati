export class NotFoundError extends Error {
  readonly type = 'NotFoundError';
  constructor(resource: string) {
    super(`${resource} not found.`);
    this.name = 'NotFoundError';
  }
}

export class InvalidInputError extends Error {
  readonly type = 'InvalidInputError';
  constructor(field: string, message: string) {
    super(`Invalid ${field} input: ${message}.`);
    this.name = 'InvalidInputError';
  }
}

export class ValidationError extends Error {
  readonly type = 'ValidationError';
  constructor(field: string, message: string) {
    super(`Invalid ${field}: ${message}.`);
    this.name = 'ValidationError';
  }
}

export class TransactionCancelled extends Error {
  readonly type = 'TransactionCancelled';
  constructor() {
    super('User cancelled the transaction.');
    this.name = 'TransactionCancelled';
  }
}

export class TransactionFailed extends Error {
  readonly type = 'TransactionFailed';
  constructor(message: string) {
    super(`Transaction failed: ${message}.`);
    this.name = 'TransactionFailed';
  }
}
