#[allow(non_camel_case_types)]
pub type ldl_int = libc::c_int;
#[allow(non_camel_case_types)]
pub type ldl_long = libc::c_long;
#[allow(non_camel_case_types)]
pub type ldl_double = libc::c_double;

pub const LDL_MAIN_VERSION: usize = 2;
pub const LDL_SUB_VERSION: usize = 1;
pub const LDL_SUBSUB_VERSION: usize = 0;

#[link(name = "ldl")]
extern "C" {
    pub fn ldl_symbolic(
        n: ldl_int,
        ap: *const ldl_int,
        ai: *const ldl_int,
        lp: *mut ldl_int,
        parent: *mut ldl_int,
        lnz: *mut ldl_int,
        flag: *mut ldl_int,
        p: *const ldl_int,
        pinv: *const ldl_int,
    );

    pub fn ldl_numeric(
        n: ldl_int,
        ap: *const ldl_int,
        ai: *const ldl_int,
        ax: *const ldl_double,
        lp: *mut ldl_int,
        parent: *mut ldl_int,
        lnz: *mut ldl_int,
        li: *mut ldl_int,
        lx: *mut ldl_double,
        d: *mut ldl_double,
        y: *mut ldl_double,
        pattern: *mut ldl_int,
        flag: *mut ldl_int,
        p: *const ldl_int,
        pinv: *const ldl_int,
    ) -> ldl_int;

    pub fn ldl_lsolve(
        n: ldl_int,
        x: *mut ldl_double,
        lp: *const ldl_int,
        li: *const ldl_int,
        lx: *const ldl_double,
    );

    pub fn ldl_dsolve(n: ldl_int, x: *mut ldl_double, d: *const ldl_double);

    pub fn ldl_ltsolve(
        n: ldl_int,
        x: *mut ldl_double,
        lp: *const ldl_int,
        li: *const ldl_int,
        lx: *const ldl_double,
    );

    pub fn ldl_perm(
        n: ldl_int,
        x: *mut ldl_double,
        b: *const ldl_double,
        p: *const ldl_int,
    );

    pub fn ldl_permt(
        n: ldl_int,
        x: *mut ldl_double,
        b: *const ldl_double,
        p: *const ldl_int,
    );

    pub fn ldl_valid_perm(
        n: ldl_int,
        p: *const ldl_int,
        flag: *const ldl_int,
    ) -> ldl_int;

    pub fn ldl_valid_matrix(
        n: ldl_int,
        ap: *const ldl_int,
        ai: *const ldl_int,
    ) -> ldl_int;

    ////////////////////////////////////////////////////////////////////////////
    //////////////////          long version           /////////////////////////
    ////////////////////////////////////////////////////////////////////////////
    pub fn ldl_l_symbolic(
        n: ldl_long,
        ap: *const ldl_long,
        ai: *const ldl_long,
        lp: *mut ldl_long,
        parent: *mut ldl_long,
        lnz: *mut ldl_long,
        flag: *mut ldl_long,
        p: *const ldl_long,
        pinv: *const ldl_long,
    );

    pub fn ldl_l_numeric(
        n: ldl_long,
        ap: *const ldl_long,
        ai: *const ldl_long,
        ax: *const ldl_double,
        lp: *mut ldl_long,
        parent: *mut ldl_long,
        lnz: *mut ldl_long,
        li: *mut ldl_long,
        lx: *mut ldl_double,
        d: *mut ldl_double,
        y: *mut ldl_double,
        pattern: *mut ldl_long,
        flag: *mut ldl_long,
        p: *const ldl_long,
        pinv: *const ldl_long,
    ) -> ldl_long;

    pub fn ldl_l_lsolve(
        n: ldl_long,
        x: *mut ldl_double,
        lp: *const ldl_long,
        li: *const ldl_long,
        lx: *const ldl_double,
    );

    pub fn ldl_l_dsolve(n: ldl_long, x: *mut ldl_double, d: *const ldl_double);

    pub fn ldl_l_ltsolve(
        n: ldl_long,
        x: *mut ldl_double,
        lp: *const ldl_long,
        li: *const ldl_long,
        lx: *const ldl_double,
    );

    pub fn ldl_l_perm(
        n: ldl_long,
        x: *mut ldl_double,
        b: *const ldl_double,
        p: *const ldl_long,
    );

    pub fn ldl_l_permt(
        n: ldl_long,
        x: *mut ldl_double,
        b: *const ldl_double,
        p: *const ldl_long,
    );

    pub fn ldl_l_valid_perm(
        n: ldl_long,
        p: *const ldl_long,
        flag: *const ldl_long,
    ) -> ldl_long;

    pub fn ldl_l_valid_matrix(
        n: ldl_long,
        ap: *const ldl_long,
        ai: *const ldl_long,
    ) -> ldl_long;
}
