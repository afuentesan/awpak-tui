import type { DataFrom } from "./data";

export type DataComparator = DataComparatorEq | 
                             DataComparatorNotEq | 
                             DataComparatorGt | 
                             DataComparatorLt | 
                             DataComparatorRegex |
                             DataComparatorAnd |
                             DataComparatorOr |
                             DataComparatorNot |
                             DataComparatorTrue |
                             DataComparatorFalse;

export enum DataComparatorVariant
{
    Eq = "Eq",
    NotEq = "NotEq",
    Gt = "Gt",
    Lt = "Lt",
    Regex = "Regex",
    And = "And",
    Or = "Or",
    Not = "Not",
    True = "True",
    False = "False"
}

export class DataComparatorEq
{
    readonly _variant = DataComparatorVariant.Eq;

    from_1 : DataFrom | undefined;
    from_2 : DataFrom | undefined;
}

export class DataComparatorNotEq
{
    readonly _variant = DataComparatorVariant.NotEq;

    from_1 : DataFrom | undefined;
    from_2 : DataFrom | undefined;
}

export class DataComparatorGt
{
    readonly _variant = DataComparatorVariant.Gt;

    from_1 : DataFrom | undefined;
    from_2 : DataFrom | undefined;
}

export class DataComparatorLt
{
    readonly _variant = DataComparatorVariant.Lt;

    from_1 : DataFrom | undefined;
    from_2 : DataFrom | undefined;
}

export class DataComparatorRegex
{
    readonly _variant = DataComparatorVariant.Regex;

    regex : string | undefined;
    from : DataFrom | undefined;
}

export class DataComparatorAnd
{
    readonly _variant = DataComparatorVariant.And;

    comp_1 : DataComparator | undefined;
    comp_2 : DataComparator | undefined;
}

export class DataComparatorOr
{
    readonly _variant = DataComparatorVariant.Or;

    comp_1 : DataComparator | undefined;
    comp_2 : DataComparator | undefined;
}

export class DataComparatorNot
{
    readonly _variant = DataComparatorVariant.Not;

    value : DataComparator | undefined;
}

export class DataComparatorTrue
{
    readonly _variant = DataComparatorVariant.True;
}

export class DataComparatorFalse
{
    readonly _variant = DataComparatorVariant.False;
}
