use typenum::{Z0, P1};
use ::{Quantity};
use ::si::{SI};

pub type Length<V> = Quantity<SI<P1, Z0, Z0, Z0, Z0, Z0, Z0>, V>;

subunits!(length; Units: Length {
    yottameter: yotta;
    zettameter: zetta;
    exameter: exa;
    petameter: peta;
    terameter: tera;
    megameter: mega;
    kilometer: kilo;
    hectometer: hecto;
    decameter: deca;
    meter: 1.0E0;
    decimeter: deci;
    centimeter: centi;
    millimeter: milli;
    micrometer: micro;
    nanometer: nano;
    picometer: pico;
    femtometer: femto;
    attometer: atto;
    zeptometer: zepto;
    yoctometer: yocto;
});
