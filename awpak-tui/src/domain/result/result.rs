use std::{marker::PhantomData, slice::Iter};

use crate::domain::error::Error;

pub struct ResultReader;
pub struct ResultValidator;
pub struct ResultWriter;
pub struct ResultFinalizer;

pub struct AwpakResult<T, U = ResultReader>
{
    inner : T,
    err : Option<Error>,
    _phantom : PhantomData<U>
}

impl<T, U> AwpakResult<T, U>
{
    pub fn own( self ) -> T
    {
        self.inner
    }

    // pub fn result_own( self ) -> Result<Self, Self>
    // {
    //     match &self.err
    //     {
    //         Some( _ ) => Err( self ),
    //         None => Ok( self )    
    //     }
    // }

    // pub fn zip<V>( self, default : Option<V>, action : impl FnOnce( &T ) -> V ) -> AwpakResult<(T, V), U>
    // {
    //     if self.err.is_some() && default.is_some()
    //     {
    //         return AwpakResult::<(T, V), U>
    //         {
    //             inner : ( self.inner, default.unwrap() ),
    //             err : self.err,
    //             _phantom : PhantomData
    //         }
    //     }

    //     let val = action( &self.inner );

    //     AwpakResult::<(T, V), U>
    //     {
    //         inner : ( self.inner, val ),
    //         err : self.err,
    //         _phantom : PhantomData
    //     }
    // }

    pub fn collect( self ) -> ( T, Option<Error> )
    {
        ( self.inner, self.err )
    }

    pub fn read( self ) -> AwpakResult<T, ResultReader>
    {
        AwpakResult::<T, ResultReader>
        { 
            inner : self.inner, 
            err: self.err, 
            _phantom : PhantomData
        }
    }

    pub fn validate( self ) -> AwpakResult<T, ResultValidator>
    {
        AwpakResult::<T, ResultValidator>
        { 
            inner : self.inner, 
            err: self.err, 
            _phantom : PhantomData
        }
    }

    pub fn write( self ) -> AwpakResult<T, ResultWriter>
    {
        AwpakResult::<T, ResultWriter>
        { 
            inner : self.inner, 
            err: self.err, 
            _phantom : PhantomData
        }
    }

    pub fn finalize( self ) -> AwpakResult<T, ResultFinalizer>
    {
        AwpakResult::<T, ResultFinalizer>
        { 
            inner : self.inner, 
            err: self.err, 
            _phantom : PhantomData
        }
    }

}

impl<T> AwpakResult<T, ResultReader>
{
    pub fn new( value : T ) -> Self
    {
        Self 
        { 
            inner : value, 
            err : None, 
            _phantom : PhantomData
        }
    }

    pub fn new_err( value : T, err : Error ) -> Self
    {
        Self 
        { 
            inner : value, 
            err : Some( err ), 
            _phantom : PhantomData
        }
    }
}

impl<T> AwpakResult<T, ResultValidator>
{
    pub fn map_result( mut self, action : impl FnOnce( &T ) -> Result<(), Error> ) -> Self
    {
        if self.err.is_some()
        {
            return self;
        }

        match action( &self.inner )
        {
            Ok( _ ) => self,
            Err( e ) =>
            {
                self.err = Some( e );

                self
            }
        }
    }

    pub fn zip_result<V>( self, action : impl FnOnce( &T ) -> Result<V, Error> ) -> AwpakResult<(T, Result<V, Error>), ResultValidator>
    {
        if self.err.is_some()
        {
            return AwpakResult::<(T, Result<V, Error>), ResultValidator>
            {
                inner : ( self.inner, Err( self.err.as_ref().unwrap().clone() ) ),
                err : self.err,
                _phantom : PhantomData
            }
        }

        match action( &self.inner )
        {
            Ok( v ) => AwpakResult::<(T, Result<V, Error>), ResultValidator>
            {
                inner : ( self.inner, Ok( v ) ),
                err : self.err,
                _phantom : PhantomData
            },
            Err( e ) => AwpakResult::<(T, Result<V, Error>), ResultValidator>
            {
                inner : ( self.inner, Err( e.clone() ) ),
                err : Some( e ),
                _phantom : PhantomData
            }
        }
    }
}

impl<T> AwpakResult<T, ResultWriter>
{
    // pub fn flat_map<V>( self, action : impl FnOnce( T ) -> AwpakResult<T, V> ) -> AwpakResult<T, V>
    // {
    //     if self.err.is_some()
    //     {
    //         return AwpakResult::<T, V>
    //         {
    //             inner : self.inner,
    //             err : self.err,
    //             _phantom : PhantomData
    //         }
    //     }

    //     action( self.inner )
    // }

    pub fn map_if( self, map : impl FnOnce( &T ) -> bool , action : impl FnOnce( T ) -> T ) -> Self
    {
        if ! map( &self.inner )
        {
            return self
        }

        self.map( action )
    }

    pub fn map( mut self, action : impl FnOnce( T ) -> T ) -> Self
    {
        if self.err.is_some()
        {
            return self;
        }

        self.inner = action( self.inner );

        self
    }

    // pub fn flat_map_on_err<V>( self, err : &Error, action : impl FnOnce( T, Error ) -> AwpakResult<T, V> ) -> AwpakResult<T, V>
    // {
    //     if self.err.is_none() || self.err.as_ref().unwrap() != err
    //     {
    //         return AwpakResult::<T, V>
    //         {
    //             inner : self.inner,
    //             err : self.err,
    //             _phantom : PhantomData
    //         }
    //     }

    //     action( self.inner, self.err.as_ref().unwrap().clone() )
    // }

    // pub fn map_on_err( mut self, err : &Error, action : impl FnOnce( T, Error ) -> T ) -> Self
    // {
    //     if self.err.is_none() || self.err.as_ref().unwrap() != err
    //     {
    //         return self;
    //     }

    //     self.inner = action( self.inner, self.err.as_ref().unwrap().clone() );

    //     self
    // }

    pub fn map_iter<U>( mut self, iter : Iter<U>, action : impl Fn( T, &U ) -> T ) -> Self
    {
        if self.err.is_some()
        {
            return self;
        }

        for u in iter
        {
            self.inner = action( self.inner, u );
        }
        
        self
    }
}

impl<T> AwpakResult<T, ResultFinalizer>
{
    pub fn unzip<V>( self, action : impl FnOnce( T ) -> V ) -> AwpakResult<V, ResultFinalizer>
    {
        let val = action( self.inner );

        AwpakResult::<V, ResultFinalizer>
        {
            inner : val,
            err : self.err,
            _phantom : PhantomData
        }
    }
}