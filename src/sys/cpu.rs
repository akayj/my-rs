use log::debug;
use raw_cpuid::{CacheType, CpuId};

pub fn cpu_cache() {
    let cpuid = CpuId::new();

    cpuid.get_cache_parameters().map_or_else(
        || println!("No cache parameter information available"),
        |cparams| {
            for cache in cparams {
                let size = cache.associativity()
                    * cache.physical_line_partitions()
                    * cache.coherency_line_size()
                    * cache.sets();

                let typ = match cache.cache_type() {
                    CacheType::Instruction => "Instruction-Cache",
                    CacheType::Data => "Data-Cache",
                    CacheType::Unified => "Unified-Cache",
                    _ => "Unknown cache type",
                };

                let associativity = if cache.is_fully_associative() {
                    "fully associative".to_string()
                } else {
                    format!("{}-way associativity", cache.associativity())
                };

                let size_repr = if size > (1 << 20) {
                    format!("{} MiB", size / (1 << 20))
                } else {
                    format!("{} KiB", size / (1 << 10))
                };

                let mapping = if cache.has_complex_indexing() {
                    "hash-based-mapping"
                } else {
                    "direct-mapped"
                };

                debug!(
                    "L{} {}: ({}, {}, {})",
                    cache.level(),
                    typ,
                    size_repr,
                    associativity,
                    mapping
                );
            }
        },
    );
}

/// cpu_info prints information of CPU, especially cpu cache.
pub fn cpu_info() {
    let cpuid = CpuId::new();

    debug!(
        "CPU Vendor: {}",
        cpuid
            .get_vendor_info()
            .as_ref()
            .map_or_else(|| "unknown", |vf| vf.as_str(),)
    );

    debug!(
        "CPU Model: {}",
        cpuid
            .get_processor_brand_string()
            .as_ref()
            .map_or_else(|| "n/a", |pbs| pbs.as_str())
    );

    debug!(
        "APIC ID: {}",
        cpuid.get_feature_info().as_ref().map_or_else(
            || String::from("n/a"),
            |finfo| format!("{}", finfo.initial_local_apic_id()),
        )
    );

    // cache info
    cpu_cache();

    // features
    let mut features = Vec::with_capacity(80);
    if let Some(finfo) = cpuid.get_feature_info() {
        if finfo.has_sse() {
            features.push("sse");
        }

        if finfo.has_sse2() {
            features.push("sse2");
        }

        if finfo.has_sse3() {
            features.push("sse3");
        }

        if finfo.has_vmx() {
            features.push("vmx");
        }

        if finfo.has_pcid() {
            features.push("pcid");
        }

        if finfo.has_fpu() {
            features.push("fpu");
        }

        if finfo.has_apic() {
            features.push("apic");
        }

        if finfo.has_mmx() {
            features.push("mmx");
        }

        if finfo.has_acpi() {
            features.push("acpi");
        }
    }

    debug!("CPU Features: {}", features.join(" "));
}
