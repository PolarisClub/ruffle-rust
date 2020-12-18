//! `flash.events.EventDispatcher` builtin/prototype

use crate::avm2::activation::Activation;
use crate::avm2::class::Class;
use crate::avm2::method::Method;
use crate::avm2::names::{Namespace, QName};
use crate::avm2::object::{DispatchObject, Object, TObject};
use crate::avm2::traits::Trait;
use crate::avm2::value::Value;
use crate::avm2::Error;
use gc_arena::{GcCell, MutationContext};

/// Implements `flash.events.EventDispatcher`'s instance constructor.
pub fn instance_init<'gc>(
    activation: &mut Activation<'_, 'gc, '_>,
    this: Option<Object<'gc>>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error> {
    if let Some(mut this) = this {
        let target = args.get(0).cloned().unwrap_or(Value::Null);
        let dispatch_list = DispatchObject::empty_list(activation.context.gc_context);

        this.init_property(
            this,
            &QName::new(Namespace::ruffle_private("EventDispatcher"), "target"),
            target,
            activation,
        )?;
        this.init_property(
            this,
            &QName::new(
                Namespace::ruffle_private("EventDispatcher"),
                "dispatch_list",
            ),
            dispatch_list.into(),
            activation,
        )?;
    }

    Ok(Value::Undefined)
}

/// Implements `EventDispatcher.addEventListener`.
pub fn add_event_listener<'gc>(
    activation: &mut Activation<'_, 'gc, '_>,
    this: Option<Object<'gc>>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error> {
    if let Some(mut this) = this {
        let dispatch_list = this
            .get_property(
                this,
                &QName::new(
                    Namespace::ruffle_private("EventDispatcher"),
                    "dispatch_list",
                ),
                activation,
            )?
            .coerce_to_object(activation)?;
        let event_type = args
            .get(0)
            .cloned()
            .unwrap_or(Value::Undefined)
            .coerce_to_string(activation)?;
        let listener = args
            .get(1)
            .cloned()
            .unwrap_or(Value::Undefined)
            .coerce_to_object(activation)?;
        let use_capture = args
            .get(2)
            .cloned()
            .unwrap_or(Value::Bool(false))
            .coerce_to_boolean();
        let priority = args
            .get(3)
            .cloned()
            .unwrap_or(Value::Integer(0))
            .coerce_to_i32(activation)?;

        //TODO: If we ever get weak GC references, we should respect `useWeakReference`.
        dispatch_list
            .as_dispatch_mut(activation.context.gc_context)
            .ok_or_else(|| Error::from("Internal properties should have what I put in them"))?
            .add_event_listener(event_type, priority, listener, use_capture);
    }

    Ok(Value::Undefined)
}

/// Implements `EventDispatcher.removeEventListener`.
pub fn remove_event_listener<'gc>(
    activation: &mut Activation<'_, 'gc, '_>,
    this: Option<Object<'gc>>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error> {
    if let Some(mut this) = this {
        let dispatch_list = this
            .get_property(
                this,
                &QName::new(
                    Namespace::ruffle_private("EventDispatcher"),
                    "dispatch_list",
                ),
                activation,
            )?
            .coerce_to_object(activation)?;
        let event_type = args
            .get(0)
            .cloned()
            .unwrap_or(Value::Undefined)
            .coerce_to_string(activation)?;
        let listener = args
            .get(1)
            .cloned()
            .unwrap_or(Value::Undefined)
            .coerce_to_object(activation)?;
        let use_capture = args
            .get(2)
            .cloned()
            .unwrap_or(Value::Bool(false))
            .coerce_to_boolean();

        dispatch_list
            .as_dispatch_mut(activation.context.gc_context)
            .ok_or_else(|| Error::from("Internal properties should have what I put in them"))?
            .remove_event_listener(event_type, listener, use_capture);
    }

    Ok(Value::Undefined)
}

/// Implements `EventDispatcher.hasEventListener`.
pub fn has_event_listener<'gc>(
    activation: &mut Activation<'_, 'gc, '_>,
    this: Option<Object<'gc>>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error> {
    if let Some(mut this) = this {
        let dispatch_list = this
            .get_property(
                this,
                &QName::new(
                    Namespace::ruffle_private("EventDispatcher"),
                    "dispatch_list",
                ),
                activation,
            )?
            .coerce_to_object(activation)?;
        let event_type = args
            .get(0)
            .cloned()
            .unwrap_or(Value::Undefined)
            .coerce_to_string(activation)?;

        return Ok(dispatch_list
            .as_dispatch_mut(activation.context.gc_context)
            .ok_or_else(|| Error::from("Internal properties should have what I put in them"))?
            .has_event_listener(event_type)
            .into());
    }

    Ok(Value::Undefined)
}

/// Implements `flash.events.EventDispatcher`'s class constructor.
pub fn class_init<'gc>(
    _activation: &mut Activation<'_, 'gc, '_>,
    _this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error> {
    Ok(Value::Undefined)
}

/// Construct `EventDispatcher`'s class.
pub fn create_class<'gc>(mc: MutationContext<'gc, '_>) -> GcCell<'gc, Class<'gc>> {
    let class = Class::new(
        QName::new(Namespace::package("flash.events"), "EventDispatcher"),
        Some(QName::new(Namespace::public_namespace(), "Object").into()),
        Method::from_builtin(instance_init),
        Method::from_builtin(class_init),
        mc,
    );

    let mut write = class.write(mc);

    write.implements(QName::new(Namespace::package("flash.events"), "IEventDispatcher").into());

    write.define_instance_trait(Trait::from_method(
        QName::new(Namespace::public_namespace(), "addEventListener"),
        Method::from_builtin(add_event_listener),
    ));
    write.define_instance_trait(Trait::from_method(
        QName::new(Namespace::public_namespace(), "removeEventListener"),
        Method::from_builtin(remove_event_listener),
    ));
    write.define_instance_trait(Trait::from_method(
        QName::new(Namespace::public_namespace(), "hasEventListener"),
        Method::from_builtin(has_event_listener),
    ));

    write.define_instance_trait(Trait::from_slot(
        QName::new(Namespace::ruffle_private("EventDispatcher"), "target"),
        QName::new(Namespace::ruffle_private(""), "BareObject").into(),
        None,
    ));
    write.define_instance_trait(Trait::from_slot(
        QName::new(
            Namespace::ruffle_private("EventDispatcher"),
            "dispatch_list",
        ),
        QName::new(Namespace::ruffle_private(""), "BareObject").into(),
        None,
    ));

    class
}
