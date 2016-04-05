//! Return each Years SigurRos Album
//! 
//!     assert_eq!(Album1999(), Ágtis_byrjun);
//!     assert_eq!(Album2002(), () );
//!     assert_eq!(Album2004(), Von);
//!     assert_eq!(Album2006(), Sæglópur);
//!     assert_eq!(Album2008(), Me_su_eyrum_vi_spilum_endalaust);
//!     assert_eq!(Album2009(), Takk);
//!     assert_eq!(Album2012(), Valtari);
//!     assert_eq!(Album2013(), Kveikur);
#![feature(non_ascii_idents)]

#[derive(Debug,PartialEq)]
pub struct Kveikur;
#[derive(Debug,PartialEq)]
pub struct Me_su_eyrum_vi_spilum_endalaust;
#[derive(Debug,PartialEq)]
pub struct Sæglópur;
#[derive(Debug,PartialEq)]
pub struct Takk;
#[derive(Debug,PartialEq)]
pub struct Valtari;
#[derive(Debug,PartialEq)]
pub struct Von;
#[derive(Debug,PartialEq)]
pub struct Ágtis_byrjun;

pub fn Album1999() -> Ágtis_byrjun{ Ágtis_byrjun }
pub fn Album2002(){}
pub fn Album2004() -> Von{ Von }
pub fn Album2006() -> Sæglópur{ Sæglópur }
pub fn Album2008() -> Me_su_eyrum_vi_spilum_endalaust{ Me_su_eyrum_vi_spilum_endalaust }
pub fn Album2009() -> Takk{ Takk }
pub fn Album2012() -> Valtari{ Valtari }
pub fn Album2013() -> Kveikur{ Kveikur }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn all_albums() {
        assert_eq!(Album1999(), Ágtis_byrjun);
        assert_eq!(Album2002(), () );
        assert_eq!(Album2004(), Von);
        assert_eq!(Album2006(), Sæglópur);
        assert_eq!(Album2008(), Me_su_eyrum_vi_spilum_endalaust);
        assert_eq!(Album2009(), Takk);
        assert_eq!(Album2012(), Valtari);
        assert_eq!(Album2013(), Kveikur);
    }

}
