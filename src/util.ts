import { Err, Ok, Result } from "@hqoss/monads"

export function errStr<T>(errorString: string): Result<T, Error> {
    return Err(new Error(errorString))
}

export function getUnknownProperty(object: unknown, key: string): Result<unknown, Error> {
    if (typeof object !== "object" || object === null) {
        return errStr("is not an object")
    }

    if (!(key in object)) {
        return errStr(`missing property '${key}'`)
    }

    // 'any' required until https://github.com/Microsoft/TypeScript/issues/21732 is implemented
    // eslint-disable-next-line @typescript-eslint/no-unsafe-member-access, @typescript-eslint/no-explicit-any
    const value: unknown = (object as any)[key]

    return Ok(value)
}

export function expectString(value: unknown): Result<string, Error> {
    if (typeof value === "string") {
        return Ok(value)
    } else {
        return errStr("Expected string")
    }
}

export function expectRecord(value: unknown): Result<Record<string, unknown>, Error> {
    if (typeof value === "object" && value !== null) {
        return Ok(value as Record<string, unknown>)
    } else {
        return errStr("Expected record")
    }
}

export function expectArray<T>(value: unknown, expectTypeFunc: (value: unknown) => Result<T, Error>): Result<Array<T>, Error> {
    if (!(value instanceof Array)) {
        return errStr("Expected Array")
    }

    // Validate value is Array<T>
    for (const x of value) {
        const result = expectTypeFunc(x)
        if (result.isErr()) {
            return Err(result.unwrapErr())
        }
    }

    return Ok(value as Array<T>)
}

export function expectNumber(value: unknown): Result<number, Error> {
    if (typeof value === "number") {
        return Ok(value)
    } else {
        return errStr("Expected number")
    }
}

export function expectSchema(input: unknown, schema: Record<string, (value: unknown) => Result<unknown, Error>>): Result<Record<string, unknown>, Error> {
    if (typeof input !== "object" || input === null) {
        return errStr("Expected record")
    }

    const record = input as Record<string, unknown>

    for (const key in schema) {
        const result = schema[key](record[key])
        if (result.isErr()) {
            return Err(result.unwrapErr())
        }
    }

    return Ok(record)
}
