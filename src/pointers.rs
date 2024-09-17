use windows::Win32::Foundation::HANDLE;
use windows::Win32::System::Diagnostics::Debug::{ReadProcessMemory, WriteProcessMemory};
use windows::Win32::System::Threading::GetCurrentProcess;

use windows::Win32::System::LibraryLoader::GetModuleHandleA;
pub struct PointerChains {
    pub pistol_ammo: Bitflag<u8>,
}

// impl Default for PointerChains {
//     fn default() -> Self {
//         Self::new()
//     }
// }

// impl PointerChains {
//     pub fn new() -> Self {
//         PointerChains {
//             pistol_ammo:
//         }
//     }
// }

pub struct Bitflag<T>(PointerChain<T>, T);

#[derive(Clone, Debug)]
pub struct PointerChain<T> {
    proc: HANDLE,
    base: *mut T,
    offsets: Vec<usize>,
}

/*



impl<T> PointerChain<T> {
    pub fn new(chain: &[usize]) -> PointerChain<T> {
        let mut it = chain.iter();
        let base = *it.next().unwrap() as *mut T;
        PointerChain {
            proc: unsafe { GetCurrentProcess() },
            base,
            offsets: it.copied().collect(), // it.map(|x| *x).collect(),
        }
    }

    fn safe_read(&self, addr: usize, offs: usize) -> Option<usize> {
        let mut value = 0usize;
        unsafe {
            ReadProcessMemory(
                self.proc,
                addr as _,
                &mut value as *mut usize as _,
                std::mem::size_of::<usize>(),
                None,
            )
            .ok()
            .map(|_| value + offs)
        }
    }

    /// Safely evaluates the pointer chain.
    /// Relies on `ReadProcessMemory` instead of pointer dereferencing for crash
    /// safety.  Returns `None` if the evaluation failed.
    pub fn eval(&self) -> Option<*mut T> {
        self.offsets
            .iter()
            .try_fold(self.base as usize, |addr, &offs| self.safe_read(addr, offs))
            .map(|addr| addr as *mut T)
    }

    pub fn read(&self) -> Option<T> {
        let ptr = self.eval()?;
        let mut value: T = unsafe { std::mem::zeroed() };
        unsafe {
            ReadProcessMemory(
                self.proc,
                ptr as _,
                &mut value as *mut _ as _,
                std::mem::size_of::<T>(),
                None,
            )
            .ok()
            .map(|_| value)
        }
    }

    /// Evaluates the pointer chain and attempts to write the datum.
    /// Returns `None` if either the evaluation or the write failed.
    pub fn write(&self, mut value: T) -> Option<()> {
        let ptr = self.eval()?;
        unsafe {
            WriteProcessMemory(
                self.proc,
                ptr as _,
                &mut value as *mut _ as _,
                std::mem::size_of::<T>(),
                None,
            )
            .ok()
            .map(|_| ())
        }
    }
}
*/
