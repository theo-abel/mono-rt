use mono_rt::{
    MonoAssembly, MonoClass, MonoDomain, MonoError, MonoImage, MonoString, MonoThreadGuard,
    TypeKind,
};

use crate::harness::Harness;

pub struct TestContext {
    pub domain: MonoDomain,
    pub fixture_path: String,
    pub player_class: MonoClass,
}

/// Navigates the Mono metadata hierarchy to produce a [`TestContext`].
///
/// # Errors
///
/// Returns [`MonoError::Uninitialized`] if any navigation step returns `None`.
pub fn setup_context(fixture_path: &str) -> Result<TestContext, MonoError> {
    let domain = require(MonoDomain::root()?)?;
    let asm = require(domain.open_assembly(fixture_path)?)?;
    let image = require(asm.image()?)?;
    let player_class = require(image.class_from_name("MonoRtTest", "Player")?)?;
    Ok(TestContext {
        domain,
        fixture_path: fixture_path.to_owned(),
        player_class,
    })
}

/// Registers all 16 test cases with the harness.
///
/// `test_thread_guard_attach_drop` is registered last: it detaches the current thread on
/// drop, so no Mono API calls may follow it.
#[allow(clippy::too_many_lines)]
pub fn register_all(h: &mut Harness, ctx: &TestContext) {
    h.run("init_succeeds", || test_init_succeeds(ctx));
    h.run("root_domain_is_some", || test_root_domain_is_some(ctx));
    h.run("fixture_assembly_loads", || {
        test_fixture_assembly_loads(ctx)
    });
    h.run("image_from_assembly", || test_image_from_assembly(ctx));
    h.run("player_class_found", || test_player_class_found(ctx));
    h.run("field_count", || test_field_count(ctx));
    h.run("health_field_type", || test_health_field_type(ctx));
    h.run("speed_field_type", || test_speed_field_type(ctx));
    h.run("active_field_type", || test_active_field_type(ctx));
    h.run("instance_count_field_type", || {
        test_instance_count_field_type(ctx)
    });
    h.run("take_damage_method_found", || {
        test_take_damage_method_found(ctx)
    });
    h.run("reset_count_method_found", || {
        test_reset_count_method_found(ctx)
    });
    h.run("method_name_round_trip", || {
        test_method_name_round_trip(ctx)
    });
    h.run("field_name_round_trip", || test_field_name_round_trip(ctx));
    h.run("mono_string_round_trip", || {
        test_mono_string_round_trip(ctx)
    });
    h.run("class_name_round_trip", || test_class_name_round_trip(ctx));
    h.run("object_get_class", || test_object_get_class(ctx));
    h.run("open_from_data_and_load_from_image", || {
        test_open_from_data_and_load_from_image(ctx)
    });
    h.run("image_open_status_message", || {
        test_image_open_status_message(ctx)
    });
    h.run("thread_guard_attach_drop", || {
        test_thread_guard_attach_drop(ctx)
    });
}

// --- helpers ----------------------------------------------------------------

fn require<T>(opt: Option<T>) -> Result<T, MonoError> {
    opt.ok_or(MonoError::Uninitialized)
}

fn field_kind(ctx: &TestContext, field_name: &str) -> Result<TypeKind, MonoError> {
    let field = require(ctx.player_class.field(field_name)?)?;
    let mono_type = require(field.mono_type()?)?;
    mono_type.kind()
}

fn assert_true(cond: bool) -> Result<(), MonoError> {
    if cond {
        Ok(())
    } else {
        Err(MonoError::Uninitialized)
    }
}

fn assert_eq_kind(actual: TypeKind, expected: TypeKind) -> Result<(), MonoError> {
    if actual == expected {
        Ok(())
    } else {
        Err(MonoError::Uninitialized)
    }
}

fn assert_eq_str(actual: &str, expected: &str) -> Result<(), MonoError> {
    if actual == expected {
        Ok(())
    } else {
        Err(MonoError::Uninitialized)
    }
}

fn assert_eq_usize(actual: usize, expected: usize) -> Result<(), MonoError> {
    if actual == expected {
        Ok(())
    } else {
        Err(MonoError::Uninitialized)
    }
}

// --- test functions ---------------------------------------------------------

fn test_init_succeeds(_ctx: &TestContext) -> Result<(), MonoError> {
    require(MonoDomain::root()?)?;
    Ok(())
}

fn test_root_domain_is_some(ctx: &TestContext) -> Result<(), MonoError> {
    assert_true(!ctx.domain.as_ptr().is_null())
}

fn test_fixture_assembly_loads(ctx: &TestContext) -> Result<(), MonoError> {
    require(ctx.domain.open_assembly(&ctx.fixture_path)?)?;
    Ok(())
}

fn test_image_from_assembly(ctx: &TestContext) -> Result<(), MonoError> {
    let asm = require(ctx.domain.open_assembly(&ctx.fixture_path)?)?;
    require(asm.image()?)?;
    Ok(())
}

fn test_player_class_found(ctx: &TestContext) -> Result<(), MonoError> {
    let asm = require(ctx.domain.open_assembly(&ctx.fixture_path)?)?;
    let image = require(asm.image()?)?;
    require(image.class_from_name("MonoRtTest", "Player")?)?;
    Ok(())
}

fn test_field_count(ctx: &TestContext) -> Result<(), MonoError> {
    assert_eq_usize(ctx.player_class.fields()?.len(), 4)
}

fn test_health_field_type(ctx: &TestContext) -> Result<(), MonoError> {
    assert_eq_kind(field_kind(ctx, "health")?, TypeKind::I4)
}

fn test_speed_field_type(ctx: &TestContext) -> Result<(), MonoError> {
    assert_eq_kind(field_kind(ctx, "speed")?, TypeKind::R4)
}

fn test_active_field_type(ctx: &TestContext) -> Result<(), MonoError> {
    assert_eq_kind(field_kind(ctx, "active")?, TypeKind::Boolean)
}

fn test_instance_count_field_type(ctx: &TestContext) -> Result<(), MonoError> {
    assert_eq_kind(field_kind(ctx, "instanceCount")?, TypeKind::I4)
}

fn test_take_damage_method_found(ctx: &TestContext) -> Result<(), MonoError> {
    require(ctx.player_class.method("TakeDamage", Some(1))?)?;
    Ok(())
}

fn test_reset_count_method_found(ctx: &TestContext) -> Result<(), MonoError> {
    require(ctx.player_class.method("ResetCount", Some(0))?)?;
    Ok(())
}

fn test_method_name_round_trip(ctx: &TestContext) -> Result<(), MonoError> {
    let method = require(ctx.player_class.method("TakeDamage", Some(1))?)?;
    assert_eq_str(&method.name()?, "TakeDamage")
}

fn test_field_name_round_trip(ctx: &TestContext) -> Result<(), MonoError> {
    let field = require(ctx.player_class.field("health")?)?;
    assert_eq_str(&field.name()?, "health")
}

fn test_mono_string_round_trip(ctx: &TestContext) -> Result<(), MonoError> {
    let s = require(MonoString::new(ctx.domain, "hello")?)?;
    assert_eq_str(&s.to_string_lossy()?, "hello")
}

fn test_class_name_round_trip(ctx: &TestContext) -> Result<(), MonoError> {
    assert_eq_str(&ctx.player_class.name()?, "Player")
}

fn test_object_get_class(ctx: &TestContext) -> Result<(), MonoError> {
    let obj = require(ctx.player_class.new_object(ctx.domain)?)?;
    let cls = require(obj.get_class()?)?;
    assert_eq_str(&cls.name()?, "Player")
}

fn test_open_from_data_and_load_from_image(ctx: &TestContext) -> Result<(), MonoError> {
    let mut bytes = std::fs::read(&ctx.fixture_path).map_err(|_| MonoError::Uninitialized)?;
    let image = MonoImage::open_from_data(&mut bytes)?;
    require(image.class_from_name("MonoRtTest", "Player")?)?;
    let asm = require(MonoAssembly::load_from_image(image, None)?)?;
    let loaded_image = require(asm.image()?)?;
    require(loaded_image.class_from_name("MonoRtTest", "Player")?)?;
    Ok(())
}

fn test_image_open_status_message(_ctx: &TestContext) -> Result<(), MonoError> {
    let msg = MonoImage::open_status_message(0)?;
    assert_true(!msg.is_empty())
}

fn test_thread_guard_attach_drop(_ctx: &TestContext) -> Result<(), MonoError> {
    // Safety: bootstrap guarantees mono_rt::init was called before any test runs.
    let guard = unsafe { MonoThreadGuard::attach()? };
    drop(guard);
    Ok(())
}
