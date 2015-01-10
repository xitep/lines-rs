#[cfg(not(target_arch = "x86_64"))]
#[inline]
/// Finds the index of the given 'needle' in 'haystack'.
pub fn index(haystack: &[u8], needle: u8) -> Option<usize> {
    haystack.iter().position(|&h| h == needle)
}

#[cfg(target_arch = "x86_64")]
#[inline(never)]
/// Finds the index of the given 'needle' in 'haystack'.
pub fn index(x: &[u8], b: u8) -> Option<usize> {
    let mut start = x.as_ptr();
    let end = unsafe {start.offset(x.len() as isize)};

    while start as usize % 16 != 0 && start < end {
        unsafe {
            if *start == b {
                return Some(start as usize - x.as_ptr() as usize);
            }
            start = start.offset(1);
        }
    }

    if (end as usize - start as usize) < 16 {
        while start < end {
            unsafe {
                if *start == b {
                    return Some(start as usize - x.as_ptr() as usize);
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

    if (start as usize) < (end as usize) {
        Some(start as usize - x.as_ptr() as usize)
    } else {
        None
    }
}

#[test]
fn test_index_byte_not_found() {
    let xs: Vec<u8> = range(1i32, 1025).map(|i| (i % 255) as u8).collect();
    assert_eq!(index(&xs[], 255), None);

    let xs = [1, 2, 3];
    assert_eq!(index(&xs[], 100), None);
}

#[test]
fn test_index() {
    let xs: Vec<u8> = range(1, 201).collect();
    let xs = xs.as_slice();

    for i in range(0, xs.len()) {
        assert_eq!(index(xs, (i+1) as u8), Some(i));
    }
}

#[test]
fn test_index_non16b_start() {
    let xs: Vec<u8> = range(1, 101).collect();
    assert_eq!(index(&xs[1..], 20), Some(18));
}
