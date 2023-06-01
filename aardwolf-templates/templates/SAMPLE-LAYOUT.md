**base_signed-out.rs.html**
```
@use gettext::Catalog;
@use crate::templates::{footer, head};

@(catalog: &Catalog, title: &str, content: Content)

<!DOCTYPE html>
<html lang="en">
    @:head(catalog, title)
    <body>
		// I am 900000% sure there's an actual syntax/better way but this is pseudocode right? ;)
        {@:sign_in_content() | @sign_up_content} // Default to sign_in, unless click sign_up Link 
    </body>
</html>
```

**sign_up_content.rs.html**
```
@use gettext::Catalog;
@use crate::templates::{footer, head};

@(catalog: &Catalog, title: &str, content: Content)

    <section class="section">
        <div class="container">

            <div class="columns is-centered">
			
				<!-- Left-Hand Column -->
                <div class="column is-5-tablet is-4-desktop is-3-widescreen">

                    <!-- ******************************* -->
                    <!-- Begin Left-Hand Column Contents -->
                    <!-- ******************************* -->

                    <h1 class="title">
                        {i18n_var="Instance Title"}
                    </h1>
                    <p class="subtitle">
                        {i18n_var="Instance Subtitle"
                    </p>
						{i18n_var="About Txt"}
                </div>
				
				<!-- Right-Hand Column -->
                <div class="column is-5-tablet is-4-desktop is-3-widescreen">

                    <!-- ******************************** -->
                    <!-- Begin Right-Hand Column Contents -->
                    <!-- ******************************** -->

                    <h1 class="title">
                        {i18n_var="Create Account"}
                    </h1>
                    <p class="subtitle">
                        {i18n_var="Feel free to sign up!"}
                    </p>

					<!-- Reusable error message block -->
                    @error_msg()

                    <form class="box" method="POST" action="/auth">
						
						@text_field_widget()
						
						@email_field_widget()
						
						@password_field_widget()
						
						@password_confirm_widget()
												
                        <button>{i18n_var="Sign Up"}</button>
                    </form>
                </div>

            </div>
        </div>
    </section>
```

**text_field_widget.rs.html**
```
<div class="field">
	<label class="label">Desired Username</label> <!-- Replace this with an i18n-variable_username -->
	<div class="control has-icons-left">
		<input class="input" type="text" placeholder="e.g. HyenaHugger" required><!-- Replace this with an i18n-variable_username_placeholder -->
		<span class=icon is-small is-left>
			<span class="fa fa-user"></span>
		</span>
	</div>
</div>
```

**email_field_widget.rs.html**
```
<div class="field">
	<label class="label">Email Address</label>
	<div class="control has-icons-left">
		<input class="input" type="email" placeholder="e.g. HyenaHugger@SmallCo.net" required>
		<span class=icon is-small is-left>
			<span class="fa fa-envelope"></span>
		</span>
	</div>
</div>
```

**password_field_widget.rs.html**
```
<div class="field">
	<label class="label">Password</label>
	<div class="control has-icons-left">
		<input class="input" type="password" placeholder="*************" required>
		<span class=icon is-small is-left>
			<span class="fa fa-lock"></span>
		</span>
	</div>
</div>
```

**password_confirm_widget.rs.html**
```
<div class="field">
	<label class="label">Confirm Password</label>
	<div class="control has-icons-left">
		<input class="input" type="password" placeholder="*************" required>
		<span class=icon is-small is-left>
			<span class="fa fa-lock"></span>
		</span>
	</div>
</div>
```
