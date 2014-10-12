#[cfg(not(target_arch = "x86_64"))]
#[inline]
/// Finds the index of the given 'needle' in 'haystack'.
pub fn index(haystack: &[u8], needle: u8) -> Option<uint> {
    haystack.iter().position(|&h| h == needle)
}

#[cfg(target_arch = "x86_64")]
#[inline(never)]
/// Finds the index of the given 'needle' in 'haystack'.
pub fn index(x: &[u8], b: u8) -> Option<uint> {
    let mut start = x.as_ptr();
    let end = unsafe {start.offset(x.len() as int)};

    while start as uint % 16 != 0 && start < end {
        unsafe {
            if *start == b {
                return Some(start as uint - x.as_ptr() as uint);
            }
            start = start.offset(1);
        }
    }

    if (end as uint - start as uint) < 16 {
        while start < end {
            unsafe {
                if *start == b {
                    return Some(start as uint - x.as_ptr() as uint);
                }
                start = start.offset(1);
            }
        }
        return None;
    }


    unsafe {
        asm! {
            "
    movq %rdx, %xmm0
    punpcklbw %xmm0, %xmm0
    punpcklbw %xmm0, %xmm0
    punpcklbw %xmm0, %xmm0
    punpcklbw %xmm0, %xmm0
    cmp %rbx, %rcx
    ja loop
    jmp end
loop:
    movapd (%rbx), %xmm1
    add $$16, %rbx
    pcmpeqb %xmm0, %xmm1
    pmovmskb %xmm1, %edx
    testl %edx, %edx
    jnz found
    cmp %rbx, %rcx
    ja loop
    jmp end
found:
    bsfw %dx, %dx
    subq $$16, %rbx
    addq %rdx, %rbx
end:"
    : "+{rbx}"(start)
    : "{rcx}"(end), "{dx}"(b)
    :"{xmm0}", "{xmm1}"
        }
    }

    if (start as uint) < (end as uint) {
        Some(start as uint - x.as_ptr() as uint)
    } else {
        None
    }
}

#[test]
fn test_index_byte_not_found() {
    let xs = Vec::<u8>::from_fn(1024, |i| ((i + 1) % 255) as u8);
    let xs = xs.as_slice();
    assert_eq!(index(xs, 255), None);

    let xs = [1, 2, 3];
    assert_eq!(index(xs, 100), None);
}

#[test]
fn test_index() {
    let xs = Vec::<u8>::from_fn(200, |i| (i + 1) as u8);
    let xs = xs.as_slice();

    for i in range(0, xs.len()) {
        assert_eq!(index(xs, (i+1) as u8), Some(i));
    }
}

#[test]
fn test_index_non16b_start() {
    let xs = Vec::<u8>::from_fn(20, |i| (i+1) as u8);
    assert_eq!(index(xs.slice_from(1), 20), Some(18));
}
