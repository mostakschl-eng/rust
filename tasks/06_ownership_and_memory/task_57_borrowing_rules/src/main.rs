fn main() {
    println!("=== AUDIT LEDGER SYNCHRONIZER ===");

    let mut audit_log = String::from("[Entry 1: System Boot]");

    println!("[Phase 1: Multi-Reader Inspection]");
    // TODO 1: Create r1 and r2 borrowing &audit_log, then print them
    let r1 = &audit_log;
    let r2 = &audit_log;
    println!("Inspector A: {}", r1);
    println!("Inspector B: {}", r2);
    // Notice: r1 and r2 are done!

    println!("\n[Phase 2: Single-Writer Update]");
    // TODO 2: Create writer borrowing &mut audit_log, append entry, and print
    let writer = &mut audit_log;
    writer.push_str(" -> [Entry 2: Backup Verified]");
    println!("Lead Auditor updated log: {}", writer);
    // Notice: writer is done!

    println!("\n[Phase 3: Final Verification]");
    // TODO 3: Create final_reader and print
    let final_reader = &audit_log;
    println!("Final Log State: {}", final_reader);

    println!("=================================");
}

