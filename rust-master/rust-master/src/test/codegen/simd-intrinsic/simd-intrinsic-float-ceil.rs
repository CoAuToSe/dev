
   -- Defer_Abort_Nestable --
   --------------------------

   procedure Defer_Abort_Nestable (Self_ID : Task_Id) is
   begin
      if No_Abort then
         return;
      end if;

      --  The following assertion is by default disabled. See the comment in
      --  Defer_Abort on the situations in which it may be useful to uncomment
      --  this assertion and enable the test.

      --  pragma Assert
      --    (Self_ID.Pending_ATC_Level >= Self_ID.ATC_Nesting_Level or else
      --     Self_ID.Deferral_Level > 0);

      Self_ID.Deferral_Level := Self_ID.Deferral_Level + 1;
   end Defer_Abort_Nestable;

   -----------------
   -- Abort_Defer --
   -----------------

   procedure Abort_Defer is
      Self_ID : Task_Id;
   begin
      if No_Abort then
         return;
      end if;

      Self_ID := STPO.Self;
      Self_ID.Deferral_Level := Self_ID.Deferral_Level + 1;
   end Abort_Defer;

   -----------------------
   -- Get_Current_Excep --
   -----------------------

   function Get_Current_Excep return SSL.EOA is
   begin
      return STPO.Self.Common.Compiler_Data.Current_Excep'Access;
   end Get_Current_Excep;

   -----------------------
   -- Do_Pending_Action --
   -----------------------

   --  Call only when holding no locks

   procedure Do_Pending_Action (Self_ID : Task_Id) is

   begin
      pragma Assert (Self_ID = Self and then Self_ID.Deferral_Level = 0);

      --  Needs loop to recheck for pending action in case a new one occurred
      --  while we had abort deferred below.

      loop
         --  Temporarily defer abort so that we can lock Self_ID

         Self_ID.Deferral_Level := Self_ID.Deferral_Level + 1;

         if Single_Lock then
            Lock_RTS;
         end if;

         Write_Lock (Self_ID);
         Self_ID.Pending_Action := False;
         Unlock (Self_ID);

         if Single_Lock then
            Unlock_RTS;
         end if;

         --  Restore the original Deferral value

         Self_ID.Deferral_Level := Self_ID.Deferral_Level - 1;

         if not Self_ID.Pending_Action then
            if Self_ID.Pending_ATC_Level < Self_ID.ATC_Nesting_Level then
               if not Self_ID.Aborting then
                  Self_ID.Aborting := True;
                  pragma Debug
                    (Debug.Trace 