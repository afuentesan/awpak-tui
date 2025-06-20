use std::{error::Error, marker::PhantomData, slice::Iter};

pub struct ResultReader;
pub struct ResultValidator;
pub struct ResultWriter;
pub struct ResultFinalizer;

pub struct AwpakResult<T, E: Error + Clone, U = ResultReader>
{
    inner : T,
    err : Option<E>,
    _phantom : PhantomData<U>
}

impl<T, E: Error + Clone, U> AwpakResult<T, E, U>
{
    pub fn own( self ) -> T
    {
        self.inner
    }

    pub fn inner_ref( &self ) -> &T
    {
        &self.inner
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

    pub fn collect( self ) -> ( T, Option<E> )
    {
        ( self.inner, self.err )
    }

    pub fn read( self ) -> AwpakResult<T, E, ResultReader>
    {
        AwpakResult::<T, E, ResultReader>
        { 
            inner : self.inner, 
            err: self.err, 
            _phantom : PhantomData
        }
    }

    pub fn validate( self ) -> AwpakResult<T, E, ResultValidator>
    {
        AwpakResult::<T, E, ResultValidator>
        { 
            inner : self.inner, 
            err: self.err, 
            _phantom : PhantomData
        }
    }

    pub fn write( self ) -> AwpakResult<T, E, ResultWriter>
    {
        AwpakResult::<T, E, ResultWriter>
        { 
            inner : self.inner, 
            err: self.err, 
            _phantom : PhantomData
        }
    }

    pub fn finalize( self ) -> AwpakResult<T, E, ResultFinalizer>
    {
        AwpakResult::<T, E, ResultFinalizer>
        { 
            inner : self.inner, 
            err: self.err, 
            _phantom : PhantomData
        }
    }

    pub fn is_ok( &self ) -> bool
    {
        self.err.is_none()
    }

    pub fn is_err( &self ) -> bool
    {
        self.err.is_some()
    }

    pub fn err( &self ) -> Option<&E>
    {
        self.err.as_ref()
    }

}

impl<T, E: Error + Clone> AwpakResult<T, E, ResultReader>
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

    pub fn new_err( value : T, err : E ) -> Self
    {
        Self 
        { 
            inner : value, 
            err : Some( err ), 
            _phantom : PhantomData
        }
    }
}

impl<T, E: Error + Clone> AwpakResult<T, E, ResultValidator>
{
    pub fn map_result( mut self, action : impl FnOnce( &T ) -> Result<(), E> ) -> Self
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

    pub fn zip_result<V>( self, action : impl FnOnce( &T ) -> Result<V, E> ) -> AwpakResult<(T, Result<V, E>), E, ResultValidator>
    {
        if self.err.is_some()
        {
            return AwpakResult::<(T, Result<V, E>), E, ResultValidator>
            {
                inner : ( self.inner, Err( self.err.as_ref().unwrap().clone() ) ),
                err : self.err,
                _phantom : PhantomData
            }
        }

        match action( &self.inner )
        {
            Ok( v ) => AwpakResult::<(T, Result<V, E>), E, ResultValidator>
            {
                inner : ( self.inner, Ok( v ) ),
                err : self.err,
                _phantom : PhantomData
            },
            Err( e ) => AwpakResult::<(T, Result<V, E>), E, ResultValidator>
            {
                inner : ( self.inner, Err( e.clone() ) ),
                err : Some( e ),
                _phantom : PhantomData
            }
        }
    }
}

impl<T, E: Error + Clone> AwpakResult<T, E, ResultWriter>
{
    pub fn flat_map<V>( self, action : impl FnOnce( T ) -> AwpakResult<T, E, V> ) -> AwpakResult<T, E, V>
    {
        if self.err.is_some()
        {
            return AwpakResult::<T, E, V>
            {
                inner : self.inner,
                err : self.err,
                _phantom : PhantomData
            }
        }

        action( self.inner )
    }

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

    pub async fn map_while( mut self, action : impl AsyncFn( T ) -> ( AwpakResult<T, E>, bool ) ) -> Self
    {
        if self.err.is_some()
        {
            return self;
        }

        loop
        {
            match action( self.inner ).await
            {
                ( r, e ) if e => match r.collect()
                {
                    ( t, None ) => self.inner = t,
                    ( t, Some( e ) ) =>
                    {
                        self.err = Some( e );

                        self.inner = t;

                        return self
                    }
                },
                ( r, _ ) => match r.collect()
                {
                    ( t, None ) =>
                    {
                        self.inner = t;

                        return self;
                    },
                    ( t, Some( e ) ) =>
                    {
                        self.err = Some( e );

                        self.inner = t;

                        return self
                    }
                }
            }    
        }
    }
}

impl<T, E: Error + Clone> AwpakResult<T, E, ResultFinalizer>
{
    pub fn unzip<V>( self, action : impl FnOnce( T ) -> V ) -> AwpakResult<V, E, ResultFinalizer>
    {
        let val = action( self.inner );

        AwpakResult::<V, E, ResultFinalizer>
        {
            inner : val,
            err : self.err,
            _phantom : PhantomData
        }
    }
}