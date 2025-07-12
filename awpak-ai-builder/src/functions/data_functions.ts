import { DataFromVariant, DataOperationAdd, DataOperationLen, DataOperationSubstract, DataOperationVariant, FromConcat, FromContext, FromInput, FromNull, FromOperation, FromParsedInput, FromStatic, type DataFrom, type DataOperation } from "../model/data";
import { DataComparatorAnd, DataComparatorEq, DataComparatorFalse, DataComparatorGt, DataComparatorLt, DataComparatorNot, DataComparatorNotEq, DataComparatorOr, DataComparatorRegex, DataComparatorTrue, DataComparatorVariant, type DataComparator } from "../model/data_comparator";
import { is_type_in_enum } from "./form_utils";

export function is_empty( data : any ) : boolean
{
    return typeof( data ) === "undefined" || data === null;
}

export function new_data_operation_variant( old : DataOperation, new_variant : string ) : DataOperation | undefined
{
    if( ! is_type_in_enum( DataOperationVariant, new_variant ) ) { return undefined; }

    new_variant = new_variant as DataOperationVariant;

    if( old._variant == new_variant ) { return old; }

    if( new_variant == DataOperationVariant.Len )
    {
        return new DataOperationLen();
    }
    else if( new_variant == DataOperationVariant.Add )
    {
        return new DataOperationAdd();
    }
    else if( new_variant == DataOperationVariant.Substract )
    {
        return new DataOperationSubstract();
    }

    return undefined;
}

export function new_data_comparator_variant( old : DataComparator, new_variant : string ) : DataComparator | undefined
{
    if( ! is_type_in_enum( DataComparatorVariant, new_variant ) ) { return undefined; }

    new_variant = new_variant as DataComparatorVariant;

    if( old._variant == new_variant ) { return old; }

    if( new_variant == DataComparatorVariant.Eq )
    {
        return new DataComparatorEq();
    }
    else if( new_variant == DataComparatorVariant.NotEq )
    {
        return new DataComparatorNotEq();
    }
    else if( new_variant == DataComparatorVariant.Gt )
    {
        return new DataComparatorGt();
    }
    else if( new_variant == DataComparatorVariant.Lt )
    {
        return new DataComparatorLt();
    }
    else if( new_variant == DataComparatorVariant.Regex )
    {
        return new DataComparatorRegex()
    }
    else if( new_variant == DataComparatorVariant.And )
    {
        return new DataComparatorAnd();
    }
    else if( new_variant == DataComparatorVariant.Or )
    {
        return new DataComparatorOr();
    }
    else if( new_variant == DataComparatorVariant.Not )
    {
        return new DataComparatorNot();
    }
    else if( new_variant == DataComparatorVariant.True )
    {
        return new DataComparatorTrue();
    }
    else if( new_variant == DataComparatorVariant.False )
    {
        return new DataComparatorFalse();
    }

    return undefined;
}

export function new_data_from_variant( old : DataFrom, new_variant : string ) : DataFrom | undefined
{
    if( ! is_type_in_enum( DataFromVariant, new_variant ) ) { return undefined; }

    new_variant = new_variant as DataFromVariant;

    if( old._variant == new_variant ) { return old; }

    if( new_variant == DataFromVariant.Context )
    {
        return new FromContext();
    }
    else if( new_variant == DataFromVariant.Input )
    {
        return new FromInput();
    }
    else if( new_variant == DataFromVariant.ParsedInput )
    {
        return new FromParsedInput();
    }
    else if( new_variant == DataFromVariant.Static )
    {
        return new FromStatic();
    }
    else if( new_variant == DataFromVariant.Operation )
    {
        return new FromOperation();
    }
    else if( new_variant == DataFromVariant.Concat )
    {
        return new FromConcat();
    }
    else if( new_variant == DataFromVariant.Null )
    {
        return new FromNull();
    }
    else
    {
        return undefined;
    }
}