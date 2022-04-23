ibling = childParentData.nextSibling;
            }

            if (childParentData.nextSibling == null) {
                D.assert(this._lastChild == child);
                this._lastChild = childParentData.previousSibling;
            } else {
                var childNextSiblingParentData = (ParentDataType) childParentData.nextSibling.parentData;
                childNextSiblingParentData.previousSibling = childParentData.previousSibling;
            }

            childParentData.previousSibling = null;
            childParentData.nextSibling = null;
            this._childCount--;
        }

        public virtual void remove(ChildType child) {
            this._removeFromChildList(child);
            this.dropChild(child);
        }

        public virtual void removeAll() {
            ChildType child = this._firstChild;
            while (child != null) {
                var childParentData = (ParentDataType) child.parentData;
                var next = childParentData.nextSibli